trait State
{
    fn local_transition(&self, neighboors: Vec<&Self>) -> Self;
}

trait Automata
{
    type State: State + Clone + Default;
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
        let n = neighboors.iter().filter_map(|state| match state
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
    type State = Etat;
    fn global_transition(&self) -> Self
    {
        let data: Vec<Etat> =
            self.data.iter().zip(0..self.data.len()).map(
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
    type State = Toto;
    fn global_transition(&self) -> Self
    {
        let data: Vec<Toto> =
            self.data.iter().zip(0..self.data.len()).map(
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



fn main() {
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
    let mut meta = MetaAutomata::new(4, 4, 4);
    for _ in 0..32
    {
        meta = meta.global_transition();
        meta.print();
        println!();
        println!();
    }
}
