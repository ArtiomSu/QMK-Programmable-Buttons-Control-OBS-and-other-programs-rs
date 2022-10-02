
pub mod programmable_keys {

    // https://docs.qmk.fm/#/feature_programmable_button
    #[derive(Debug)]
    pub enum ProgrammableKeys {
        MACROUNKOWN = 0,
        MACRO1 = 656,
        MACRO2 = 657,
        MACRO3 = 658,
        MACRO4 = 659,
        MACRO5 = 660,
        MACRO6 = 661,
        MACRO7 = 662,
        MACRO8 = 663,
        MACRO9 = 664,
        MACRO10 = 665,
        MACRO11 = 666,
        MACRO12 = 667,
        MACRO13 = 668,
        MACRO14 = 669,
        MACRO15 = 670,
        MACRO16 = 671,
        MACRO17 = 672,
        MACRO18 = 673,
        MACRO19 = 674,
        MACRO20 = 675,
        MACRO21 = 676,
        MACRO22 = 677,
        MACRO23 = 678,
        MACRO24 = 679,
        MACRO25 = 680,
        MACRO26 = 681,
        MACRO27 = 682,
        MACRO28 = 683,
        MACRO29 = 684,
        MACRO30 = 685,
        MACRO31 = 686,
        MACRO32 = 687
    }

    impl ProgrammableKeys {
        pub fn from_u32(value: u32) -> ProgrammableKeys {
            match value {
                656 => ProgrammableKeys::MACRO1,
                657 => ProgrammableKeys::MACRO2,
                658 => ProgrammableKeys::MACRO3,
                659 => ProgrammableKeys::MACRO4,
                660 => ProgrammableKeys::MACRO5,
                661 => ProgrammableKeys::MACRO6,
                662 => ProgrammableKeys::MACRO7,
                663 => ProgrammableKeys::MACRO8,
                664 => ProgrammableKeys::MACRO9,
                665 => ProgrammableKeys::MACRO10,
                666 => ProgrammableKeys::MACRO11,
                667 => ProgrammableKeys::MACRO12,
                668 => ProgrammableKeys::MACRO13,
                669 => ProgrammableKeys::MACRO14,
                670 => ProgrammableKeys::MACRO15,
                671 => ProgrammableKeys::MACRO16,
                672 => ProgrammableKeys::MACRO17,
                673 => ProgrammableKeys::MACRO18,
                674 => ProgrammableKeys::MACRO19,
                675 => ProgrammableKeys::MACRO20,
                676 => ProgrammableKeys::MACRO21,
                677 => ProgrammableKeys::MACRO22,
                678 => ProgrammableKeys::MACRO23,
                679 => ProgrammableKeys::MACRO24,
                680 => ProgrammableKeys::MACRO25,
                681 => ProgrammableKeys::MACRO26,
                682 => ProgrammableKeys::MACRO27,
                683 => ProgrammableKeys::MACRO28,
                684 => ProgrammableKeys::MACRO29,
                685 => ProgrammableKeys::MACRO30,
                686 => ProgrammableKeys::MACRO31,
                687 => ProgrammableKeys::MACRO32,
                _ => {
                    println!("Unknown programmable key {}", value);
                    ProgrammableKeys::MACROUNKOWN
                },
            }
        }

        async fn obs_change_scene(client: &Option<obws::Client>, scene_name: &str) {
            match client {
                Some(client) => {
                    match client.scenes().set_current_program_scene(scene_name).await {
                        Ok(_) => {
                            println!("Scene {} set", scene_name);
                        },
                        Err(e) => {
                            eprintln!("Error setting scene: {:?}", e);
                        }
                    }
                },
                None => {},
            }
        }

        async fn obs_recording_toggle_pause(client: &Option<obws::Client>) {
            match client {
                Some(client) => {
                    match client.recording().toggle_pause().await {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error toggling pause: {:?}", e);
                        }
                    }
                },
                None => {},
            }
        }

        async fn send_post_request(url: &str, body: &String) {
            let message = body.clone();
            let client = reqwest::Client::new();
            match client.post(url)
                            .body(message)
                            .header("Content-Type", "application/json")
                            .send()
                            .await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error sending post request: {:?}", e);
                },
            };
        }

        pub async fn process_keys(key: ProgrammableKeys, client: &Option<obws::Client>) {
            match key {
                ProgrammableKeys::MACRO1 => {
                    println!("MACRO1 PRESSED");
                    ProgrammableKeys::obs_change_scene(client, "Scene").await;
                },
                ProgrammableKeys::MACRO2 => {
                    println!("MACRO2 PRESSED");
                    ProgrammableKeys::obs_change_scene(client, "Scene 2").await;
                },
                ProgrammableKeys::MACRO3 => {
                    println!("MACRO3 PRESSED");
                    ProgrammableKeys::obs_change_scene(client, "Scene 3").await;
                },
                ProgrammableKeys::MACRO4 => {
                    println!("MACRO4 PRESSED");
                    let body:String = String::from("{\"on\":true,\"button\":\"1\",\"sleepduration\":-1}");
                    ProgrammableKeys::send_post_request("http://10.0.0.12:9090/operate", &body).await;
                },
                ProgrammableKeys::MACRO5 => {
                    println!("MACRO5 PRESSED");
                    let body:String = String::from("{\"on\":false,\"button\":\"1\",\"sleepduration\":-1}");
                    ProgrammableKeys::send_post_request("http://10.0.0.12:9090/operate", &body).await;
                },
                ProgrammableKeys::MACRO6 => {
                    println!("MACRO6 PRESSED");
                    let body:String = String::from("{\"on\":true,\"button\":\"3\",\"sleepduration\":180}");
                    ProgrammableKeys::send_post_request("http://10.0.0.12:9090/timeout", &body).await;
                },
                ProgrammableKeys::MACRO7 => {
                    println!("MACRO7 PRESSED");
                },
                ProgrammableKeys::MACRO8 => {
                    println!("MACRO8 PRESSED");
                },
                ProgrammableKeys::MACRO9 => {
                    println!("MACRO9 PRESSED");
                    ProgrammableKeys::obs_recording_toggle_pause(client).await;
                },
                ProgrammableKeys::MACRO10 => {
                    println!("MACRO10 PRESSED");
                    let body:String = String::from("{\"button\":\"KEY_A\"}");
                    ProgrammableKeys::send_post_request("http://10.0.0.12:9090/ir", &body).await;
                },
                ProgrammableKeys::MACRO11 => {
                    println!("MACRO11 PRESSED");
                    let body:String = String::from("{\"button\":\"KEY_B\"}");
                    ProgrammableKeys::send_post_request("http://10.0.0.12:9090/ir", &body).await;
                },
                ProgrammableKeys::MACRO12 => {
                    println!("MACRO12 PRESSED");
                },
                ProgrammableKeys::MACRO13 => {
                    println!("MACRO13 PRESSED");
                },
                ProgrammableKeys::MACRO14 => {
                    println!("MACRO14 PRESSED");
                },
                ProgrammableKeys::MACRO15 => {
                    println!("MACRO15 PRESSED");
                },
                ProgrammableKeys::MACRO16 => {
                    println!("MACRO16 PRESSED");
                },
                ProgrammableKeys::MACRO17 => {
                    println!("MACRO17 PRESSED");
                },
                ProgrammableKeys::MACRO18 => {
                    println!("MACRO18 PRESSED");
                },
                ProgrammableKeys::MACRO19 => {
                    println!("MACRO19 PRESSED");
                },
                ProgrammableKeys::MACRO20 => {
                    println!("MACRO20 PRESSED");
                },
                ProgrammableKeys::MACRO21 => {
                    println!("MACRO21 PRESSED");
                },
                ProgrammableKeys::MACRO22 => {
                    println!("MACRO22 PRESSED");
                },
                ProgrammableKeys::MACRO23 => {
                    println!("MACRO23 PRESSED");
                },
                ProgrammableKeys::MACRO24 => {
                    println!("MACRO24 PRESSED");
                },
                ProgrammableKeys::MACRO25 => {
                    println!("MACRO25 PRESSED");
                },
                ProgrammableKeys::MACRO26 => {
                    println!("MACRO26 PRESSED");
                },
                ProgrammableKeys::MACRO27 => {
                    println!("MACRO27 PRESSED");
                },
                ProgrammableKeys::MACRO28 => {
                    println!("MACRO28 PRESSED");
                },
                ProgrammableKeys::MACRO29 => {
                    println!("MACRO29 PRESSED");
                },
                ProgrammableKeys::MACRO30 => {
                    println!("MACRO30 PRESSED");
                },
                ProgrammableKeys::MACRO31 => {
                    println!("MACRO31 PRESSED");
                },
                ProgrammableKeys::MACRO32 => {
                    println!("MACRO32 PRESSED");
                },
                _ => {
                    println!("MACROUNKOWN PRESSED");
                }
            }
        }
    }
}