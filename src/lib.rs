
turbo::cfg!(
    r#"
    name = "Cube Drawer"
    version = "1.0.0"
    author = "Turbo"
    description = "Draw a cube at the left side."
    [settings]
    resolution = [800, 400]
"#
);

turbo::init! {
    struct GameState {
        frame: u32,
        t: u32,
        gob_x: f32,
        gob_y: f32,
        gob_r: f32,
        score: u32,
        burger_x : f32,
        burger_y : f32,
        burger_r : f32,
        burger_removed: bool,
        
    } = {
        Self {
            frame: 0,
            score: 0,
            gob_x: 400.0,
            gob_y: 300.0,
            gob_r: 16.0,
            t:0,
            burger_x : 300.0,
            burger_y : 10.0,
            burger_r : 16.0,
            burger_removed: false,
        }
    }
}

turbo::go! {
    let mut state = GameState::load();

    

    let startm = (0., 100.);
    let endm = (400., 350.);
    let peak_height = -64.;
    let progress = (state.t % 100) as f32 / 100.;
    let pm = interpolate_parabolic(startm, endm, peak_height, progress);
   
    let startn = (100., 400.);
    let endn = (400., 128.);
    let pn = interpolate_parabolic(startn, endn, peak_height, progress);

    let starto = (150., 350.);
    let endo = (350., 100.);
    let po = interpolate_parabolic(starto, endo, peak_height, progress);

    let startp = (500., 100.);
    let endp = (100., 500.);
    let pp = interpolate_parabolic(startp, endp, peak_height, progress);

    if pm.0 == state.gob_x || pn.0 == state.gob_x || po.0 == state.gob_x|| pp.0 == state.gob_x{

        text!("GAME OVER YOU SYCK!", x= 400, y = 200, font = Font::L, color = 0xffffffff);

          
        
    }
    else {
        rect!(fill = 0xff00ffff, x = pm.0 as i32, y = pm.1 as i32, w = 8, h = 8);
        rect!(fill = 0xff00ffff, x = pp.0 as i32, y = pp.1 as i32, w = 8, h = 8);
        rect!(fill = 0xff00ffff, x = po.0 as i32, y = po.1 as i32, w = 8, h = 8);
        rect!(fill = 0xff00ffff, x = pn.0 as i32, y = pn.1 as i32, w = 8, h = 8);
        state.t += 1;
    }


    

    // let start = (150., 350.);
    // let end = (350., 100.);
    // let peak_height = -64.;
    // let progress = (state.t % 100) as f32 / 100.;
    // let p = interpolate_parabolic(start, end, peak_height, progress);
    // rect!(fill = 0xff00ffff, x = p.0 as i32, y = p.1 as i32, w = 8, h = 8);
    // state.t += 1;


    


    if gamepad(0).left.pressed() {
        state.gob_x += 2.;
    }
    if gamepad(0).right.pressed() {
        state.gob_x -= 2.;
    }

    if gamepad(0).up.pressed() {
        state.gob_y += 1.5;
    }
    if gamepad(0).down.pressed() {
        state.gob_y -= 1.5;
    }

    if mouse(0).left.pressed() {
        state.gob_x -= 9.;     
    }

    if mouse(0).right.pressed() {
        state.gob_x += 9.;       
    } 


    sprite!("burger", x = 300, y = 10);
    sprite!("goblin_1", x = (state.gob_x - state.gob_r) as i32, y = (state.gob_y - state.gob_r) as i32, fps = fps::FAST);


    // Draw your cooking game screen
    text!("Eat that burgurr!" , font = Font::L, color = 0xffffffff);

    



    fn is_collision(gob_x: f32, gob_y: f32, gob_r: f32, burger_x: f32, burger_y: f32) -> bool {
        let dx = gob_x - burger_x;
        let dy = gob_y - burger_y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Check if the distance between the goblin and burger is less than their combined radii
        distance < 35.0 // Assuming the burger has a radius of 16.0
    }
      

    if  is_collision(state.gob_x, state.gob_y, state.gob_r, state.burger_x, state.burger_y) {
        // Goblin touched the burger

        text!("You Win!", x = 300, y = 200, font = Font::L, color = 0xffffffff);
    }

    

    state.save();
}



fn interpolate_parabolic(
    p0: (f32, f32),
    p1: (f32, f32),
    peak_height: f32,
    progress: f32,
) -> (f32, f32) {
    // Linear interpolation for x-coordinate
    let x = p0.0 * (1.0 - progress) + p1.0 * progress;

    // Parabolic interpolation for y-coordinate
    let parabolic_progress = 4.0 * peak_height * progress * (1.0 - progress);
    let y = p0.1 * (1.0 - progress) + p1.1 * progress + parabolic_progress;

    // Return the interpolated point
    (x, y)
}

