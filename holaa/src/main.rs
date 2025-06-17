// Esta "fn" es una funcion y "main" es el punto de entrada del programa

fn main() {
    println!("Hello, world!");

    // let es para definir variable, "mut" sirve paara hacer cambios en la variable, {variable}
    // aqui va nuestra variable

    let x: i32 = 10;

    //let mut x: i32 = 10; 
    //x = 20;
    //aplicamos mut para modificar

    println!("x: {x}");

    //aqui es lo de la funcion productostemu. le puse asi para no confundirme con lo de ingles

    println!("resultado carnal: {}", productostemu(120, 100, 248));

    //esto es de "numerocon_x" 

    let k = 10;
    let l = 20;

    numerocon_1(k);
    numerocon_2(l);
    //aqui si termina el de numerocons

    //Funcion nuevavida

    nuevavida();

    //Funcion elbabodetemu

    elbabodetemu();
}
fn productostemu(a: i32, b: i32, c: i32) -> i32 {
        return a * b * c + c * a;    
}

// Variables y Inferencia de tipos

fn numerocon_1(k: u32){
    println!("el de u32 carnal: {k}")
}

fn numerocon_2(l: i8){
    println!("el de i8 carnal: {l}")
}

//Nueva etapa como la cancion de nueva vida de Peso Deidad

fn nuevavida(){

    //Esta es la variable

    let yoyo = 12;

    // si yoyo es cero, imprime esto:

    if yoyo == 0{
        println!("Cero carnal, ni modo");

        // si yoyo es menor que 100, imprime esto:

    } else if yoyo < 100 {
        println!("wey, nmms. El {yoyo} es menor que el 100. Es grande el 100 we, ya mijo ya");

        //si hay un numero mas grande, pues...

    } else {
        println!("No, ni de broma. Es grandote");
    }
}

//Otra funcion similar al uso de if y else y eso:

fn elbabodetemu(){
    let polloyconejo = 7;
    //Se aplica con if polloyconejo < 100, devuevlve a "pequeño" porque 7 es pequeño, y si no lo fuera da "else" a grandote

    let tamano = if polloyconejo < 100 {"pequeño"} else {"grandote"};

    //aqui se imprime
    
    println!("tamaño del numero por Dharius: {}", tamano);
}