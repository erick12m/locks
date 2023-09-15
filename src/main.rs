use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};

const PAN_0: u32 = 0;
const PAN_1: u32 = 1;
const LECHUGA: u32 = 2;
const CARNE: u32 = 3;

const PRECIO_PAN: i32 = 10;
const PRECIO_LECHUGA: i32 = 5;
const PRECIO_CARNE: i32 = 30;

const MAX_ORO: i32 = 20;

fn minero(oro: Arc<RwLock<i32>>, id: i32) {
    loop {
        let produccion = (rand::random::<i32>() % MAX_ORO).abs();
        *oro.write().unwrap() += produccion;
        //println!("Minero {} produjo: {} oro", id, produccion);
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn comprador(oro: Arc<RwLock<i32>>, id: i32, recursos: Arc<RwLock<HashMap<String, i32>>>) {
    loop {
        let accion = rand::random::<u32>() % 4;
        match accion {
            PAN_0 | PAN_1 => {
                let mut recursos = recursos.write().unwrap();
                let mut oro = oro.write().unwrap();
                if *oro >= PRECIO_PAN {
                    *oro -= PRECIO_PAN;
                    *recursos.entry("PAN".to_string()).or_insert(0) += 1;
                    //println!("comprador {} compro pan", id);
                }
            }
            LECHUGA => {
                let mut recursos = recursos.write().unwrap();
                let mut oro = oro.write().unwrap();
                if *oro >= PRECIO_LECHUGA {
                    *oro -= PRECIO_LECHUGA;
                    *recursos.entry("LECHUGA".to_string()).or_insert(0) += 1;
                    //println!("comprador {} compro lechuga", id);
                }
            }
            CARNE => {
                let mut recursos = recursos.write().unwrap();
                let mut oro = oro.write().unwrap();
                if *oro >= PRECIO_CARNE {
                    *oro -= PRECIO_CARNE;
                    *recursos.entry("CARNE".to_string()).or_insert(0) += 1;
                    //println!("comprador {} compro carne", id);
                }
            }
            _ => {}
        }
        thread::sleep(std::time::Duration::from_secs(2));
    }
}

fn cocinero(id: i32, recursos: Arc<RwLock<HashMap<String, i32>>>) {
    loop {
        {
            let mut recursos = recursos.write().unwrap();
            if recursos.get("PAN").unwrap_or(&0) >= &2
                && recursos.get("LECHUGA").unwrap_or(&0) >= &1
                && recursos.get("CARNE").unwrap_or(&0) >= &1
            {
                *recursos.entry("PAN".to_string()).or_insert(0) -= 2;
                *recursos.entry("LECHUGA".to_string()).or_insert(0) -= 1;
                *recursos.entry("CARNE".to_string()).or_insert(0) -= 1;
                //println!("Cocinero {} hizo hamburguesa", id);
                *recursos.entry("HAMBURGUESA".to_string()).or_insert(0) += 1;
            }
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn consumidor(oro: Arc<RwLock<i32>>, id: i32, recursos: Arc<RwLock<HashMap<String, i32>>>) {
    loop {
        {
            let mut recursos = recursos.write().unwrap();
            if recursos.get("HAMBURGUESA").unwrap_or(&0) >= &1 {
                *recursos.entry("HAMBURGUESA".to_string()).or_insert(0) -= 1;
                {
                    let mut oro = oro.write().unwrap();
                    *oro += 100;
                }
                //println!("Consumidor {} comio hamburguesa", id);
            }
        }
        thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn main() {
    let oro = Arc::new(RwLock::new(0));
    let recursos = Arc::new(RwLock::new(HashMap::new()));

    let mineros: Vec<JoinHandle<()>> = (0..2)
        .map(|id| {
            let oro_aux = oro.clone();
            thread::spawn(move || minero(oro_aux, id))
        })
        .collect();

    let compradores: Vec<JoinHandle<()>> = (0..3)
        .map(|id| {
            let oro_aux = oro.clone();
            let recursos_aux = recursos.clone();
            thread::spawn(move || comprador(oro_aux, id, recursos_aux))
        })
        .collect();

    let cocineros: Vec<JoinHandle<()>> = (0..1)
        .map(|id| {
            let recursos_aux = recursos.clone();
            thread::spawn(move || cocinero(id, recursos_aux))
        })
        .collect();

    let consumidores: Vec<JoinHandle<()>> = (0..3)
        .map(|id| {
            let oro_aux = oro.clone();
            let recursos_aux = recursos.clone();
            thread::spawn(move || consumidor(oro_aux, id, recursos_aux))
        })
        .collect();

    let logger: JoinHandle<()> = thread::spawn(move || loop {
        println!("Oro: {}", oro.read().unwrap());
        println!("Recursos: {:?}", recursos.read().unwrap());
        thread::sleep(std::time::Duration::from_secs(10));
    });

    mineros.into_iter().for_each(|m| m.join().unwrap());
    compradores.into_iter().for_each(|c| c.join().unwrap());
    cocineros.into_iter().for_each(|c| c.join().unwrap());
    consumidores.into_iter().for_each(|c| c.join().unwrap());
    logger.join().unwrap();
}
