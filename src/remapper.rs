use std::sync::{Arc, Mutex, mpsc::Receiver};

use enigo::Enigo;
use gilrs::{Gilrs, Event, EventType::*};

use crate::{config::Config, button::update_button, axis::update_axis};


pub fn update_config(current_config: Arc<Mutex<Config>>, rx: Receiver<Config>) {
    // On écoute en permanence pour changer de config active
    // Mais on ne peut en pratique en changer
    // Que lorsque le remap n'est pas activé
    loop {
        // Opération blocante qui va être appelée très rarement
        // C'est celle qui fait qu'on ne peut pas changer pendant que le remap est activé
        // Car on y utilise et lock le mutex de la config
        let config = rx.recv().unwrap();
        let mut guard = current_config.lock().unwrap();
        *guard = config;
        // On drop le guard pour pouvoir relancer le remap
        drop(guard);
    }
}

pub fn launch(current_config: Arc<Mutex<Config>>, rx: Receiver<bool>) {
    // À utiliser lors de la création du thread pour remap
    // Utiliser le channel pour run / resume (true / false)
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new();
    let mut run = false;
    loop {
        while run {
            let mut current_config = current_config.lock().unwrap();
            while let Some(Event { id: _, event, time: _ }) = gilrs.next_event() {
                match event {
                    ButtonPressed(button, _) => {
                        update_button(&mut enigo, button, true, &mut current_config);
                    },
        
                    ButtonReleased(button, _) => {
                        update_button(&mut enigo, button, false, &mut current_config);
                    },
        
                    AxisChanged(axis, value, _) => {
                        update_axis(&mut enigo, axis, value, &mut current_config);
                    },
    
                    ButtonRepeated(_, _) | ButtonChanged(_, _, _) | Connected | Disconnected | Dropped => ()
                }
            }
            // Il ne faut pas bloquer cette boucle puisqu'on veut remplacer
            // en temps réel tous les inputs manettes par des inputs claviers
            run = rx.try_recv().is_err();
        }
        // On ne run plus donc on peut se permettre d'être bloquant
        run = rx.recv().unwrap();
    }
}