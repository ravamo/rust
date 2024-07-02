fn main() {
    // if-let
        let edad :Option <i32> = Some(20);
            match edad {
                Some(value) => println!("edad: {}",value),
                _=>(),
            }   
            
            if let Some(value) = edad {
               println!("edad: {}",value);
            }
            
            
            // white-let
            
            let mut sms = Some(10);
             loop {
                 match sms {
                     Some(value) => {
                         if value > 0 {
                            println!("algo");
                            sms = Some(value -1);
                            
                         } else {
                            println!("no hace algo");
                            sms = None;
                             
                         }
                     }
                     _ =>{break;}
                 }
             }
             
            while let Some(value) = sms {
                                 if value > 0 {
                            println!("algo");
                            sms = Some(value -1);
                            
                         } else {
                            println!("no hace algo");
                            sms = None;
                             
                         }
                
            }
    }