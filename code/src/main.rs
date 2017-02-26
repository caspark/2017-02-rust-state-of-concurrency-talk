#[derive(Debug, Clone, Eq, PartialEq)]
struct Starscape {
    stars: Vec<Star>,
    cat: NyanCat,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct NyanCat {
    pos: (i32, i32),
    volume: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct Star {
    pos: (i32, i32),
}

fn main() {
    {
        let universe: Starscape = unimplemented!(); // somehow make a Starscape
    }

    {
        let cat: NyanCat = unimplemented!(); //somehow make a cat

        let cat2 = cat;

        println!("{:?}", cat);
        println!("{:?}", cat2);
    }

    {
        let mut cat: NyanCat = unimplemented!(); //somehow make a cat

        let cat2: &mut NyanCat = &mut cat;

        cat2.volume = 5;
    }
}