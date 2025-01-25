use raylib::prelude::*;
use raylib::core::texture::Texture2D;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

pub struct Locale {
    code: String,
    language: String,
    texture: Option<Texture2D>,
    translations: HashMap<String, String>,
}

impl Locale {
    pub fn load(path: &str) -> Result<Vec<Self>, String> {
        let mut reader: Reader<BufReader<File>> = Reader::from_file(path).unwrap();
        reader.config_mut().trim_text(true);

        let mut locales: Vec<Locale> = Vec::new();
        let mut buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"language" => {
                            locales.push(Self { 
                                code: Self::read_attribute(&e, b"code")?, 
                                language: Self::read_text(&mut reader)?,
                                texture: None,
                                translations: HashMap::new(), 
                            });
                        },
                        _ => (),
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(format!("Error parsing XML at position {}: {:?}", reader.error_position(), e)),
                _ => (),
            }
            buf.clear();
        }

        for locale in locales.iter_mut() {
            locale.load_from_xml().expect("Error parsing locale XML");
        }

        Ok(locales)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_language(&self) -> &String {
        &self.language
    }

    pub fn get_texture(&self) -> &Option<Texture2D> {
        &self.texture
    }

    pub fn load_texture(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let filename: String = format!("assets/images/locales/{}.png", self.get_code());
        let img: Image = Image::load_image(&filename).unwrap();
        self.texture = Some(rl.load_texture_from_image(thread, &img).unwrap());
    }

    pub fn unload_texture(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if self.texture.is_none() { return; }
        
        let mut texture: Option<Texture2D> = None;
        std::mem::swap(&mut self.texture, &mut texture);
        unsafe {
            rl.unload_texture(thread, texture.unwrap().make_weak());
        }
    }

    fn load_from_xml(&mut self) -> Result<bool, String> {
        let path: String = format!("assets/locales/{}.xml", self.code);
        let mut reader: Reader<BufReader<File>> = Reader::from_file(&path).unwrap();
        reader.config_mut().trim_text(true);

        let mut buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"string" => {
                            let key = Self::read_attribute(&e, b"name")?;
                            let value = Self::read_text(&mut reader)?;
                            self.translations.insert(key, value);
                        },
                        _ => (),
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(format!("Error parsing XML at position {}: {:?}", reader.error_position(), e)),
                _ => (),
            }
            buf.clear();
        }

        Ok(true)
    }

    fn read_text(reader: &mut Reader<BufReader<File>>) -> Result<String, String> {
        let mut buf = Vec::new();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => Ok(e.unescape().unwrap().into_owned()),
            _ => Err("Expected text".to_string()),
        }
    }
    
    fn read_attribute(e: &BytesStart, name: &[u8]) -> Result<String, String> {
        let attr_value_result = e.attributes()
            .filter(|a| a.as_ref().unwrap().key.0 == name)
            .map(|a| a.unwrap().unescape_value().map_err(|e| e.to_string()))
            .last().ok_or_else(|| "Attribute not found".to_string());

        match attr_value_result {
            Ok(x) => Ok(x?.into_owned()),
            Err(x) => Err(x),
        }
    }
}