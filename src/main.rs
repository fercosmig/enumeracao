enum Direction
{
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
enum Gender
{
    Male,
    Female
}

enum Carro
{
    Fiat,
    Ford,
    Renault
}

enum Pagamento
{
    Dinheiro(f32),
    Credito(bool, f32),
    Debito(bool, f32)
}


fn nacionalidade_carro(car: Carro)
{
    match car
    {
        Carro::Fiat => println!("Carro italiano"),
        Carro::Ford => println!("Carro americano"),
        Carro::Renault => println!("Carro francês")
    }
}

fn main() {

    // exemplo 1
    let player: Direction;

    player = Direction::Right;

    match player
    {
        Direction::Up => println!("O jogador foi para cima"),
        Direction::Down => println!("O jogador foi para baixo"),
        Direction::Left => println!("O jogador foi para a esquerda"),
        Direction::Right => println!("O jogador foi para a direita")
    }

    // exemplo 2
    let player1: Gender;
    let player2: Gender;

    player1 = Gender::Male;
    player2 = Gender::Female;

    println!("P1: {:?}", player1);
    println!("P2: {:?}", player2);

    // exemplo 3
    nacionalidade_carro(Carro::Fiat);
    nacionalidade_carro(Carro::Ford);
    nacionalidade_carro(Carro::Renault);

    // exemplo 4
    let pessoa_pagamento: Pagamento;

    pessoa_pagamento = Pagamento::Credito(true, 100.5);

    match pessoa_pagamento
    {
        Pagamento::Dinheiro(valor) => println!("Pago em dinheiro. R$ {}", valor),
        Pagamento::Credito(true, valor) => println!("Pago com cartão de crédito. R$ {}", valor),
        Pagamento::Credito(false, valor) => println!("Falha no pagamento com cartão de crédito. R$ {}", valor),
        Pagamento::Debito(true, valor) => println!("Pago com cartão de débito. R$ {}", valor),
        Pagamento::Debito(false, valor) => println!("Falha no pagamento com cartão de débito. R$ {}", valor),
    }
}
