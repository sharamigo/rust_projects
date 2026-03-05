#[derive(Clone, Debug, PartialEq)]
pub struct GpsCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl GpsCoordinate {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            latitude: lat,
            longitude: lon,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Etappe {
    pub id: u32,
    pub name: String,
    pub beschreibung: String,
    pub laenge_km: f64,
    pub steigung_hm: i32,
    pub abstieg_hm: i32,
    pub startort: String,
    pub zielort: String,
    pub start_gps: GpsCoordinate,
    pub ziel_gps: GpsCoordinate,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Wandertour {
    pub id: u32,
    pub name: String,
    pub beschreibung: String,
    pub schwierigkeitsgrad: String,
    pub etappen: Vec<Etappe>,
    pub images: Vec<TourImage>,
}

impl Wandertour {
    pub fn gesamtlaenge(&self) -> f64 {
        self.etappen.iter().map(|e| e.laenge_km).sum()
    }
    
    pub fn gesamtsteigung(&self) -> i32 {
        self.etappen.iter().map(|e| e.steigung_hm).sum()
    }
    
    pub fn gesamtabstieg(&self) -> i32 {
        self.etappen.iter().map(|e| e.abstieg_hm).sum()
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct TourImage {
    pub src: String,
    pub alt: String,
    pub caption: String,
}