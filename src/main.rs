use ::function_name::named;

#[derive(Debug)]
struct Traveller<'a>{
    name:  &'a str,
    location: &'a str
}

impl<'a> Drop for Traveller<'a>{
    fn drop(&mut self) {
        println!("Traveller {} is now leaving {}", self.name, self.location);
    }
}

#[named]
fn rome(){
    let _t1 = Traveller{name: "Bob", location: function_name!()};
}

#[named]
fn brisbane(){
    let _t1 = Traveller{name: "Natalie", location: function_name!()};
    {
        let _t2 = Traveller{name: "Rob", location: function_name!()}; 
    }
}

#[named]
fn casablanca<'a>() -> Traveller<'a>{
    let t1 = Traveller{name: "Ken", location: function_name!()};
    return t1;
}

#[named]
fn houston(traveller: &mut Traveller){
    println!("Traveller {} arrived from: {}", traveller.name, function_name!());
    traveller.location =  function_name!();
}

#[named]
fn main(){

    println!();

    let _t1 = Traveller{name: "Jil", location: function_name!()};

    rome();
    brisbane();
    houston(&mut casablanca());

}