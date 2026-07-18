use std::{
    cell::{Cell, RefCell},
    fs::File,
    path::{Path, PathBuf},
    time::Duration,
};

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};

#[derive(Clone)]
pub struct Track {
    pub path: PathBuf,
    pub title: String,
    pub duration: Duration,
}

impl Track {
    /// Próbuje zdekodować plik, żeby poznać jego długość. Zwraca None dla
    /// plików, których nie da się odtworzyć.
    fn probe(path: PathBuf) -> Option<Self> {
        if !path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("mp3"))
        {
            return None;
        }

        let file = File::open(&path).ok()?;
        let byte_len = file.metadata().map(|meta| meta.len()).unwrap_or(0);

        let source = Decoder::builder()
            .with_data(file)
            .with_byte_len(byte_len)
            .build()
            .ok()?;

        let title = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_else(|| String::from("Nieznany utwór"));

        Some(Self {
            path,
            title,
            duration: source.total_duration().unwrap_or(Duration::ZERO),
        })
    }
}

pub struct AudioPlayer {
    // OutputStream musi żyć tak długo jak Sink - po jego zrzuceniu dźwięk znika.
    _stream: OutputStream,
    sink: Sink,
    music_dir: PathBuf,
    tracks: RefCell<Vec<Track>>,
    current: Cell<usize>,
}

impl AudioPlayer {
    /// Wczytuje wszystkie pliki .mp3 z katalogu (posortowane po nazwie)
    /// i od razu zaczyna odtwarzać pierwszy z nich.
    pub fn from_dir(dir: impl AsRef<Path>) -> Self {
        let music_dir = dir.as_ref().to_path_buf();

        let mut paths: Vec<PathBuf> = std::fs::read_dir(&music_dir)
            .expect("Brak katalogu z muzyką")
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect();
        paths.sort();

        let tracks: Vec<Track> = paths.into_iter().filter_map(Track::probe).collect();
        assert!(!tracks.is_empty(), "Brak plików .mp3 w katalogu");

        let stream =
            OutputStreamBuilder::open_default_stream().expect("Brak urządzenia audio");
        let sink = Sink::connect_new(stream.mixer());

        let player = Self {
            _stream: stream,
            sink,
            music_dir,
            tracks: RefCell::new(tracks),
            current: Cell::new(0),
        };
        // Ładuj pierwszy utwór, ale nie odtwarzaj - dopiero po kliknięciu Play
        player.load(0, false);
        player
    }

    /// Zatrzymuje bieżący utwór i ładuje ten o podanym indeksie playlisty.
    /// Gdy `play` jest true, od razu zaczyna odtwarzanie.
    fn load(&self, index: usize, play: bool) {
        let path = self.tracks.borrow()[index].path.clone();

        let file = File::open(&path).expect("Nie można otworzyć pliku audio");
        let byte_len = file.metadata().map(|meta| meta.len()).unwrap_or(0);

        // with_byte_len pozwala symphonii policzyć długość utworu i włącza seek
        let source = Decoder::builder()
            .with_data(file)
            .with_byte_len(byte_len)
            .with_gapless(true)
            .build()
            .expect("Nie można zdekodować pliku audio");

        self.current.set(index);

        // clear() usuwa zakolejkowane źródła i pauzuje sink
        self.sink.clear();
        self.sink.append(source);
        if play {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }

    /// Kopiuje plik .mp3 do `assets/music` (jeśli jeszcze go tam nie ma)
    /// i dodaje go do playlisty. Zwraca true, jeśli się udało.
    pub fn add_track(&self, path: PathBuf) -> bool {
        let dest = match self.copy_into_music_dir(&path) {
            Some(dest) => dest,
            None => return false,
        };

        // Nie dodawaj drugi raz tego samego pliku
        if self.tracks.borrow().iter().any(|track| track.path == dest) {
            return false;
        }

        match Track::probe(dest) {
            Some(track) => {
                self.tracks.borrow_mut().push(track);
                true
            }
            None => false,
        }
    }

    /// Kopiuje plik do katalogu z muzyką. Jeśli plik już tam jest, zwraca
    /// jego ścieżkę bez kopiowania. Przy kolizji nazw dodaje sufiks `_1`, `_2`...
    fn copy_into_music_dir(&self, path: &Path) -> Option<PathBuf> {
        if !path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("mp3"))
        {
            return None;
        }

        // Plik już leży w katalogu z muzyką - nie kopiuj drugi raz
        if path.parent().is_some_and(|parent| parent == self.music_dir) {
            return Some(path.to_path_buf());
        }

        let file_name = path.file_name()?;
        let mut dest = self.music_dir.join(file_name);

        if dest.exists() {
            let stem = path.file_stem()?.to_string_lossy();
            let mut n = 1u32;
            loop {
                dest = self.music_dir.join(format!("{stem}_{n}.mp3"));
                if !dest.exists() {
                    break;
                }
                n += 1;
            }
        }

        std::fs::copy(path, &dest).ok()?;
        Some(dest)
    }

    /// Kopia playlisty do wyświetlenia w UI.
    pub fn tracks(&self) -> Vec<Track> {
        self.tracks.borrow().clone()
    }

    /// Następny utwór (zapętla się na koniec playlisty).
    pub fn next(&self) {
        self.load((self.current.get() + 1) % self.tracks.borrow().len(), true);
    }

    /// Poprzedni utwór (zapętla się na początek playlisty).
    pub fn previous(&self) {
        let index = self
            .current
            .get()
            .checked_sub(1)
            .unwrap_or(self.tracks.borrow().len() - 1);
        self.load(index, true);
    }

    /// Odtwarza bieżący utwór od początku.
    pub fn restart(&self) {
        if self.sink.empty() {
            // Utwór dograł do końca - trzeba go załadować ponownie
            self.load(self.current.get(), true);
        } else {
            self.seek(Duration::ZERO);
            self.sink.play();
        }
    }

    pub fn play(&self) {
        if self.sink.empty() {
            self.load(self.current.get(), true);
        } else {
            self.sink.play();
        }
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume.clamp(0., 1.));
    }

    pub fn seek(&self, position: Duration) {
        let _ = self.sink.try_seek(position);
    }

    pub fn position(&self) -> Duration {
        self.sink.get_pos()
    }

    pub fn is_playing(&self) -> bool {
        !self.sink.is_paused() && !self.sink.empty()
    }

    /// Utwór dograł do końca (sink nie ma już żadnego źródła).
    pub fn is_finished(&self) -> bool {
        self.sink.empty()
    }

    pub fn title(&self) -> String {
        self.tracks.borrow()[self.current.get()].title.clone()
    }

    pub fn duration(&self) -> Duration {
        self.tracks.borrow()[self.current.get()].duration
    }
}
