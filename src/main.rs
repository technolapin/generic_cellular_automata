/**
I'm currently trying to make some generic cellular automatas
Testing the use of the Traits "State" and "Automata" with the usual Conway's game of life (Toto)
and an automata whose states are Conway's game of life
*/

use rayon::prelude::*;
use rayon::iter::* ;
trait State
{
    fn local_transition(&self, neighboors: Vec<&Self>) -> Self;
}

trait Automata
{
    type Cell: State + Clone + Default;
    fn global_transition(&self) -> Self;
    
}





#[derive(Clone, Debug, PartialEq)]
enum Etat
{
    ON,
    OFF
}

impl Etat
{
    fn to_string(&self) -> String
    {
        match self
        {
            Etat::ON => "O".to_string(),
            Etat::OFF => "·".to_string()
        }
    }

}

impl Default for Etat
{
    fn default() -> Self
    {
        Etat::OFF
    }
}

impl State for Etat
{
    fn local_transition(&self, neighboors: Vec<&Self>) -> Self
    {
        let n = neighboors.par_iter().filter_map(|state| match state
                                             {
                                                 Etat::ON => Some(()),
                                                 Etat::OFF => None
                                             }).count();
        if n < 2 || n > 3
        {
            Etat::OFF
        }
        else if n == 3
        {
            Etat::ON
        }
        else
        {
            self.clone()
        }
    }
}

#[derive(Clone)]
struct Toto
{
    data: Vec<Etat>,
    w: usize,
    h: usize,
    heat: usize
}

impl Default for Toto
{
    fn default() -> Self
    {
        Self::new(0, 0)
    }
}

impl Toto
{
    fn new(w: usize, h: usize) -> Self
    {
        Toto
        {
            data: (0..(w*h)).map(|_| Etat::OFF).collect(),
            w: w,
            h: h,
            heat: 0
        }
    }

    fn random(w: usize, h: usize) -> Self
    {
        Toto
        {
            data: (0..(w*h)).map(|_| if rand::random() {Etat::OFF} else {Etat::ON}).collect(),
            w: w,
            h: h,
            heat: 0
        }
    }

    fn print(&self)
    {
        println!("-----------------------------------------");
        println!("{}", self.heat);
        for j in 0..self.h
        { 
            for i in 0..self.w
            {
                print!("{}", self.data[i+j*self.w].to_string())
            }
            println!();
        }
    }
    fn turn_on(&mut self, x: usize, y: usize)
    {
        self.data[x + self.w*y] = Etat::ON
    }
}

impl Automata for Toto
{
    type Cell = Etat;
    fn global_transition(&self) -> Self
    {
        let data: Vec<Etat> =
            self.data.par_iter().zip(0..self.data.len()).map(
                |(cell, index)|
                {
                    let i = index as isize;
                    let w = self.w as isize;
                    
                    cell.local_transition(
                        vec![i-w-1, i - w, i-w+1,
                             i - 1       , i + 1,
                             i+w-1, i + w, i+w+1
                        ].iter().filter_map(
                            |&jndex|
                            if jndex < 0 || jndex >= self.data.len() as isize
                            {
                                None
                            }
                            else
                            {
                                Some(&self.data[jndex as usize])
                            }
                        ).collect()
                    )
                        
                }
            ).collect();

        let heat = self.data.iter().zip(data.iter()).fold(0, |count, (state_a, state_b)|
                                                          if *state_a == *state_b
                                                          {
                                                              count
                                                          }
                                                          else
                                                          {
                                                              count+1
                                                          });
        Toto
        {
            data: data,
            w: self.w,
            h: self.h,
            heat: heat
        }
    }
}


impl State for Toto
{
    fn local_transition(&self, neighboors: Vec<&Self>) -> Self
    {
        if self.heat >= neighboors.iter().map(|&automate| automate.heat).max().unwrap()
        {
            self.global_transition()
        }
        else
        {
            self.clone()
        }
    }
}


struct MetaAutomata
{
    data: Vec<Toto>,
    w: usize,
    h: usize
}


impl Automata for MetaAutomata
{
    type Cell = Toto;
    fn global_transition(&self) -> Self
    {
        let data: Vec<Toto> =
            self.data.par_iter().zip(0..self.data.len()).map(
                |(cell, index)|
                {
                    let i = index as isize;
                    let w = self.w as isize;
                    
                    self.data[index].local_transition(
                        vec![i-w-1, i - w, i-w+1,
                             i - 1       , i + 1,
                             i+w-1, i + w, i+w+1
                        ].iter().filter_map(
                            |&jndex|
                            if jndex < 0 || jndex >= self.data.len() as isize
                            {
                                None
                            }
                            else
                            {
                                Some(&self.data[jndex as usize])
                            }
                        ).collect()
                    )
                        
                }
            ).collect();
        MetaAutomata
        {
            data: data,
            w: self.w,
            h: self.h
        }
    }
}


impl MetaAutomata
{
    fn new(w: usize, h: usize, pas: usize) -> Self
    {
        MetaAutomata
        {
            data: (0..(w*h)).map(|_| Toto::random(pas, pas) ).collect(),
            w: w,
            h: h
        }
    }

    fn print(&self)
    {
        println!("{}", "-".repeat(self.w*(self.data[0].w+1)));
        
        for j in 0..self.h
        {
            for y in 0..self.data[0].h
            {
                print!("|");
                for i in 0..self.w
                {
                    for x in 0..self.data[0].w
                    {
                        let local_index = x + y*self.data[0].w;
                        print!("{}", self.data[i+j*self.w].data[local_index].to_string());
                    }
                    print!("|")
                }
                println!();
            }
            println!("{}", "-".repeat(self.w*(self.data[0].w+1)+1));
        }
    }
}

enum Direction
{
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}

#[derive(Clone, Default)]
struct LightRay
{
    intensity: f64,
    direction: usize,
}

#[derive(Clone)]
struct Block
{
    light: Vec<LightRay>,
    diffusion: f64,
    opacity: f64,
}

impl State for Block
{
    fn local_transition(&self, neighboors: Vec<&Self>) -> Self
    {
        Block
        {
            light:
            neighboors.iter().zip(0..neighboors.len()).map(
                |(block, side)|
                {
                    let direction = (4 + side) % 8;
                    LightRay
                    {
                        intensity: block.light.iter().filter_map(
                            |ray|
                            if ray.intensity < 1.0 {None}
                            else if ray.direction == direction
                            {
                                Some(ray.intensity*(1.-self.opacity-self.diffusion*7.))
                            }
                            else
                            {
                                Some(ray.intensity*self.diffusion)
                            }
                        ).fold(0., |total_intens, intens| total_intens+intens),
                        direction: direction
                    }
                }
            ).collect(),
            diffusion: self.diffusion,
            opacity: self.opacity
        }
    }
}


struct LightAutomata
{
    data: Vec<Block>,
    w: usize,
    h: usize

}
impl Block
{
    fn new(opacity: f64, diffusion: f64) -> Self
    {
        Self
        {
            light: vec![],
            opacity: opacity,
            diffusion: diffusion
        }
    }
}
impl Default for Block
{
    fn default() -> Self
    {
        Self
        {
            light: vec![],
            opacity: 0.,
            diffusion: 0.
        }
    }
    
}


impl Automata for LightAutomata
{
    type Cell = Block;
    fn global_transition(&self) -> Self
    {
        let default_cell = Block::default();
        let data: Vec<Self::Cell> =
            self.data.iter().zip(0..self.data.len()).map(
                |(cell, index)|
                {
                    let i = index as isize;
                    let w = self.w as isize;
                    
                    self.data[index].local_transition(
                        vec![i-w-1, i - w, i-w+1,
                             i + 1       , i - 1,
                             i+w+1, i + w, i+w-1
                        ].iter().filter_map(
                            |&jndex|
                            if jndex < 0 || jndex >= self.data.len() as isize
                            {
                                Some(&default_cell)
                            }
                            else
                            {
                                Some(&self.data[jndex as usize])
                            }
                        ).collect()
                    )
                        
                }
            ).collect();
        Self
        {
            data: data,
            w: self.w,
            h: self.h
        }
    }
    
}

impl LightAutomata
{
    fn new(w: usize, h: usize, opacity: f64, diffuse: f64) -> Self
    { 
        Self
        {
            data: (0..(w*h)).map(|_| Block::new(opacity, diffuse) ).collect(),
            w: w,
            h: h
        }
    }
    
    fn print(&self)
    {
        for j in 0..self.h
        {
            for i in 0..self.w
            {
                let intensity = self.data[i+j*self.w].light.iter().fold(0., |intens, ray| intens+ray.intensity);
                print!("{} ", intensity as u64);
            }
            println!();
        }
        
    }

    fn draw(&self, window: &mut orbclient::Window, pas: u32)
    {
        for i in 0..self.w
        {
            for j in 0..self.h
            {
                let intensity = self.data[i+j*self.w].light.iter().fold(0., |intens, ray| intens+ray.intensity);
                let color = intensity.min(255.) as u8;
                let x = i as i32;
                let y = j as i32;
                window.rect(x*(pas as i32), y*(pas as i32), pas, pas, Color::rgba(0, color, color, 255));
            }
        }
        window.sync();

    }
}

#[derive(Default, Clone)]
struct Enlighted
{
    diffusion: f64,
    opacity: f64,
    rays: [f64; 4] // left, right, up, down
}

impl Enlighted
{
    fn new(opacity: f64, diffusion: f64) -> Self
    {
        Self
        {
            rays: [0., 0., 0., 0.],
            diffusion: diffusion,
            opacity: opacity
        }
    }

}



impl State for Enlighted
{
    fn local_transition(&self, nghbrs: Vec<&Self>) -> Self // LRUD
    {
        Enlighted{
            diffusion: self.diffusion,
            opacity: self.opacity,
            rays: [
                (nghbrs[0].rays[0]*(1.-3.0*self.diffusion) + (nghbrs[0].rays[1] + nghbrs[0].rays[2] + nghbrs[0].rays[3])*self.diffusion)* self.opacity,
                (nghbrs[1].rays[1]*(1.-3.0*self.diffusion) + (nghbrs[1].rays[0] + nghbrs[1].rays[2] + nghbrs[1].rays[3])*self.diffusion)* self.opacity,
                (nghbrs[2].rays[2]*(1.-3.0*self.diffusion) + (nghbrs[2].rays[0] + nghbrs[2].rays[1] + nghbrs[2].rays[3])*self.diffusion)* self.opacity,
                (nghbrs[3].rays[3]*(1.-3.0*self.diffusion) + (nghbrs[3].rays[0] + nghbrs[3].rays[1] + nghbrs[3].rays[2])*self.diffusion)* self.opacity,
            ]
        }
    }
}

struct EnlightAutomata
{
    data: Vec<Enlighted>,
    w: usize,
    h: usize
}

impl EnlightAutomata
{
    fn new(w: usize, h: usize, opacity: f64, diffusion: f64) -> Self
    {
        EnlightAutomata
        {
            data: (0..(w*h)).map(|_| Enlighted::new(opacity, diffusion) ).collect(),
            w: w,
            h: h
        }
    }
    
    fn get_cell(&self, x: isize, y: isize) -> Option<&Enlighted>
    {
        if x < 0 || y < 0 || x >= self.w as isize || y >= self.h as isize
        {
            None
        }
        else
        {
            Some(&self.data[x as usize + y as usize*self.w])
        }
    }
   fn draw(&self, window: &mut orbclient::Window, pas: u32)
    {
        for i in 0..self.w
        {
            for j in 0..self.h
            {
                let intensity: f64 = self.data[i+j*self.w].rays.iter().fold(0., |intens, val| val*val).sqrt();
                let color = intensity.min(255.) as u8;
                let x = i as i32;
                let y = j as i32;
                window.rect(x*(pas as i32), y*(pas as i32), pas, pas, Color::rgba(color, color, (color as f32 * 0.8) as u8, 255));
            }
        }
        window.sync();

    }

}


impl Automata for EnlightAutomata
{
    type Cell = Enlighted;
    fn global_transition(&self) -> Self
    {
        let border_cell = Self::Cell::default();
        let data: Vec<_> = self.data.par_iter().enumerate().map(
            |(i, cell)|
            {
                let x = (i % self.w) as isize;
                let y = (i / self.w) as isize;
                cell.local_transition(
                    vec![
                        (-1, 0), (1, 0), (0, -1), (0, 1)
                    ].iter().map(
                        |(dx, dy)| match self.get_cell(x+dx, y+dy)
                        {
                            None => &border_cell,
                            Some(c) => c
                        }
                    ).collect()
                )
            }).collect();
        Self
        {
            w: self.w,
            h: self.h,
            data: data
        }
    }
}


fn test1() {
    use Etat::{ON, OFF};
    let voisins = vec![&ON, &ON, &OFF, &OFF, &ON, &OFF, &OFF];
    println!("{:?}", OFF.local_transition(voisins));

    let mut toto = Toto::new(8, 8);
    toto.turn_on(2, 0);
    toto.turn_on(2, 1);
    toto.turn_on(2, 2);
    toto.turn_on(1, 2);
    toto.turn_on(0, 1);
    toto.print();
    toto = toto.global_transition();
    toto.print();
    toto = toto.global_transition();
    toto.print();
    toto = toto.global_transition();
    toto.print();
    toto = toto.global_transition();
    toto.print();
    println!("WEEEEEEEEEEEEEEE");
    let mut meta = MetaAutomata::new(8, 4, 16);
//    for _ in 0..320
    loop
    {
        meta = meta.global_transition();
        meta.print();
        for i in 0..999999 {}
        println!();
        println!();
    }
}



fn test2()
{
    
    let mut clair = LightAutomata::new(16, 16, 0.01, 0.02);
    //    for _ in 0..320
    let lit = Block
    {
        light: vec![
            LightRay
            {
                intensity: 10.,
                direction: 5
            }
        ],
        opacity: 0.01,
        diffusion: 0.01
    };
    loop
    {
        clair.data[clair.h/2*clair.w+clair.w/2] = lit.clone();
        clair = clair.global_transition();
        clair.print();
        println!();
        println!();
    }
}
use orbclient::{Color, EventOption, GraphicsPath, Mode, Renderer, Window};


fn test3()
{
    let mut clair = LightAutomata::new(100, 100, 0.001, 0.1);
    let mut x_mouse = 0;
    let mut y_mouse = 0;

    let pas = 4;
    let window_width = pas*(clair.w as u32);
    let window_height = pas*(clair.h as u32);
    let mut window = Window::new_flags(
        0,
        0,
        window_width,
        window_height,
        "TITLE",
        &[
            orbclient::WindowFlag::Transparent,
            orbclient::WindowFlag::Async,
        ]
    )
    .unwrap();


    let lit = Block
    {
        light: (0..8).map(
            |i|
            LightRay
            {
                intensity: 250.,
                direction: i
            }
        ).collect(),
        opacity: 0.1,
        diffusion: 0.01
    };
    'mainloop: loop
    {
        match window.events().next()
        {
            Some(event) =>
            {
                match event.to_option()
                {
                    EventOption::Quit(_quit_event) => break 'mainloop,
                    EventOption::Mouse(mouse_event) =>
                    {
                        let (x, y) = (mouse_event.x / (pas as i32), mouse_event.y/(pas as i32));
                        if x >= 0 && x < clair.w as i32 && y >= 0 && y < clair.h as i32
                        {
                            x_mouse = x as usize;
                            y_mouse = y as usize;
                            println!("enlighting at {} {}", x, y);
                        }
                    },
                    _ => ()

                }
            },
            None =>
            {
                clair.data[x_mouse + y_mouse*clair.w] = lit.clone();
                clair = clair.global_transition();
                clair.draw(&mut window, pas);
                println!("test");
                
            }
            
        }
    }

}


fn test4()
{
    let mut clair = EnlightAutomata::new(200, 200, 0.99, 0.2);
//    let mut clair = EnlightAutomata::new(200, 200, 0.62, 0.2);
    let mut x_mouse = 0;
    let mut y_mouse = 0;

    let pas = 4;
    let window_width = pas*(clair.w as u32);
    let window_height = pas*(clair.h as u32);
    let mut window = Window::new_flags(
        0,
        0,
        window_width,
        window_height,
        "TITLE",
        &[
            orbclient::WindowFlag::Transparent,
            orbclient::WindowFlag::Async,
        ]
    )
    .unwrap();

    let intensity = 200000.;
    let lit = Enlighted
    {
        rays: [intensity, intensity, intensity, intensity],
        opacity: clair.data[0].opacity,
        diffusion: clair.data[0].diffusion,
        
    };
    let mut is_lit = true;
    
    'mainloop: loop
    {
        match window.events().next()
        {
            Some(event) =>
            {
                match event.to_option()
                {
                    EventOption::Quit(_quit_event) => break 'mainloop,
                    EventOption::Mouse(mouse_event) =>
                    {
                        let (x, y) = (mouse_event.x / (pas as i32), mouse_event.y/(pas as i32));
                        if x >= 0 && x < clair.w as i32 && y >= 0 && y < clair.h as i32
                        {
                            x_mouse = x as usize;
                            y_mouse = y as usize;
                            println!("enlighting at {} {}", x, y);
                        }
                    },
                    EventOption::Button(event) => {
                        is_lit = !is_lit;
                    },
                    _ => println!("{:?}", event)

                }
            },
            None =>
            {
                for _ in 0..1
                {
                    if is_lit {clair.data[x_mouse + y_mouse*clair.w] = lit.clone()};
                    clair = clair.global_transition();
                }
                clair.draw(&mut window, pas);
                
            }
            
        }
    }

}

fn main()
{
    test4();
}
