fn main() {
    {
        struct GrayscaleMap {
            pixels: Vec<u8>,
            size: (usize, usize),
        }
        
        fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
            assert_eq!(pixels.len(), size.0 * size.1);
            GrayscaleMap { pixels, size }
        }
        
        let width = 1024;
        let height = 768;
        let image = GrayscaleMap {
            pixels: vec![0; width * height],
            size: (width, height),
        };
    
        assert_eq!(image.size.0, 1024);
    
        let image = new_map((width, height), vec![0; width * height]);
    
        assert_eq!(image.size.1, 768);
        assert_eq!(image.pixels.len(), 1024 * 768);
    }

    {
        #[allow(dead_code)]
        struct Broom {
            name: String,
            height: u32,
            health: u32,
            position: (f32, f32, f32),
            intent: BroomIntent,
        }

        #[derive(Copy, Clone)]
        #[allow(dead_code)]
        enum BroomIntent { FetchWater, DumpWater }

        fn chomp(b: Broom) -> (Broom, Broom) {
            let mut broom1 = Broom { height: b.height / 2, .. b };

            let mut broom2 = Broom { name: broom1.name.clone(), .. broom1 };

            broom1.name.push_str(" I");
            broom2.name.push_str(" II");

            (broom1, broom2)
        }

        let hokey = Broom {
            name: "Hokey".to_string(),
            height: 60,
            health: 100,
            position: (100.0, 200.0, 0.0),
            intent: BroomIntent::FetchWater,
        };

        let (hokey1, hokey2) = chomp(hokey);
        
        assert_eq!(hokey1.name, "Hokey I");
        assert_eq!(hokey1.height, 30);
        assert_eq!(hokey1.health, 100);
 
        assert_eq!(hokey2.name, "Hokey II");
    }
}
