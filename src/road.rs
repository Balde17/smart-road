pub use sdl2::rect::Point;
pub use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;

pub fn draw_road(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    canvas.fill_rect(Rect::new(275, 0, 250, 1000)).unwrap();
    
    canvas.fill_rect(Rect::new(0, 270, 800, 250)).unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    
    let mut dashed_line_y = 0;
    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(320, dashed_line_y), Point::new(320, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 20;
    }

    let mut dashed_line_y = 0;
    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(360, dashed_line_y), Point::new(360, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 20;
    }
    
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let mut dashed_line_y = 0;
    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(400, dashed_line_y), Point::new(400, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 20;
    }

    canvas.set_draw_color(Color::RGB(0, 255, 255));

    let mut dashed_line_y = 0;
    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(440, dashed_line_y), Point::new(440, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 20;
    }

    let mut dashed_line_y = 0;
    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(480, dashed_line_y), Point::new(480, dashed_line_y + 10))
            .unwrap();
        dashed_line_y += 20;
    }

    
    dashed_line_y = 1000;
    for _ in 0..24 { 
        canvas
            .draw_line(Point::new(320, dashed_line_y), Point::new(320, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 20;
    }

    dashed_line_y = 1000;
    for _ in 0..24 { 
        canvas
            .draw_line(Point::new(360, dashed_line_y), Point::new(360, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 20;
    }
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    dashed_line_y = 1000;
    for _ in 0..24 { 
        canvas
            .draw_line(Point::new(400, dashed_line_y), Point::new(400, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 20;
    }
    canvas.set_draw_color(Color::RGB(0, 255, 255));

    dashed_line_y = 1000;
    for _ in 0..24 { 
        canvas
            .draw_line(Point::new(440, dashed_line_y), Point::new(440, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 20;
    }

    dashed_line_y = 1000;
    for _ in 0..24 { 
        canvas
            .draw_line(Point::new(480, dashed_line_y), Point::new(480, dashed_line_y - 10))
            .unwrap();
        dashed_line_y -= 20;
    }



    let mut dashed_line_x = 0;
    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(dashed_line_x, 320), Point::new(dashed_line_x + 10, 320))
            .unwrap();
        dashed_line_x += 20;
    }

    let mut dashed_line_x = 0;

    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(dashed_line_x, 360), Point::new(dashed_line_x + 10, 360))
            .unwrap();
        dashed_line_x += 20;
    }
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let mut dashed_line_x = 0;

    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(dashed_line_x, 400), Point::new(dashed_line_x + 10, 400))
            .unwrap();
        dashed_line_x += 20;
    }

    canvas.set_draw_color(Color::RGB(0, 255, 255));

    let mut dashed_line_x = 0;

    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(dashed_line_x, 440), Point::new(dashed_line_x + 10, 440))
            .unwrap();
        dashed_line_x += 20;
    }

    let mut dashed_line_x = 0;
    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(dashed_line_x, 480), Point::new(dashed_line_x + 10, 480))
            .unwrap();
        dashed_line_x += 20;
    }

    let mut dashed_line_x = 800;
    for _ in 0..14 {
        canvas
            .draw_line(Point::new(dashed_line_x, 320), Point::new(dashed_line_x + 10, 320))
            .unwrap();
        dashed_line_x -= 20;
    }

    let mut dashed_line_x = 800;

    for _ in 0..14 {
        canvas
            .draw_line(Point::new(dashed_line_x, 360), Point::new(dashed_line_x + 10, 360))
            .unwrap();
        dashed_line_x -= 20;
    }
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let mut dashed_line_x = 800;

    for _ in 0..14 {
        canvas
            .draw_line(Point::new(dashed_line_x, 400), Point::new(dashed_line_x + 10, 400))
            .unwrap();
        dashed_line_x -= 20;
    }

    canvas.set_draw_color(Color::RGB(0, 255, 255));

    let mut dashed_line_x = 800;

    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(dashed_line_x, 440), Point::new(dashed_line_x + 10, 440))
            .unwrap();
        dashed_line_x -= 20;
    }

    let mut dashed_line_x = 800;
    for _ in 0..14 { 
        canvas
            .draw_line(Point::new(dashed_line_x, 480), Point::new(dashed_line_x + 10, 480))
            .unwrap();
        dashed_line_x -= 20;
    }
    
    // Dessiner deux lignes continues délimitant les côtés gauche et droite de la route
    canvas.draw_line(Point::new(275, 0), Point::new(275, 1000)).unwrap();
    canvas.draw_line(Point::new(525, 0), Point::new(525, 1000)).unwrap();
    
    // Dessiner deux lignes continues délimitant les côtés supérieur et inférieur de la route
    canvas.draw_line(Point::new(0, 270), Point::new(800, 270)).unwrap();
    canvas.draw_line(Point::new(0, 520), Point::new(800, 520)).unwrap();
    
}
