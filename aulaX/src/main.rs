//Exercício 1: Adicionando Elementos a um Vetor
fn main() {
    let mut vetor = Vec::new();
    
    vetor.push(5);
    vetor.push(10);
    vetor.push(15);
    
    println!("{:?",vetor); // Deve imprimir [5, 10, 15]
    }
    
    //Exercício 2: Acessando Elementos de um Vetor
    fn main() {
        let frutas = vec!["maça", "banana", "jaca", "tomate", "pitaya"];
        
        let primeira = frutas [1];
        let ultima = frutas[frutas.len() - 3];
            
        println!("primeira fruta: {}", primeira);
        println!("ultima fruta: {}", ultima);
    }
    
    //Exercício 3: Verificando o Tamanho de um Vetor
    Crie um vetor e verifique quantos elementos ele possui.
    fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let tamanho = numeros.len();
    
    println!("O vetor tem {} elementos.", tamanho);
    }
    
    //Exercício 4: Removendo o Último Elemento
    Crie um vetor e remova o último elemento. Depois, imprima o vetor resultante.
    fn main() {
    let mut vetor = vec![10, 20, 30];
    vetor.pop(); // Remove o último elemento
    
    println!("{:?}", vetor); // Deve imprimir [10, 20]
    }
    
    //Exercício 5: Somando os Elementos de um Vetor
    Crie um vetor de números inteiros e calcule a soma de todos os elementos.
    fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let soma: i32 = numeros.iter().sum();
    
    println!("A soma dos elementos é:", soma);
    }
    
    //Exercício 6: Verificando se um Elemento Existe
    Crie um vetor de strings e verifique se uma determinada string está presente.
    fn main() {
    let frutas = vec!["maçã", "banana", "laranja"];
    let procura = "banana";
    
    if frutas.contains(&procura) {
    println!("{} está no vetor.", procura);
    } else {
    println!("{} não está no vetor.", procura);
    }
    }
    
    //Exercício 7: Criando um Vetor com Valores Repetidos
    Crie um vetor onde todos os elementos são o mesmo valor.
    fn main() {
    let vetor = vec![0; 5]; // Cria um vetor com 5 elementos, todos iguais a 0
    
    println!("{:?}", vetor); // Deve imprimir [0, 0, 0, 0, 0]
    }
    
    //Exercício 8: Iterando com Índices
    Crie um vetor e imprima cada elemento junto com seu índice.
    
    fn main() {
    let frutas = vec!["maçã", "banana", "laranja"];
    
    for (indice, fruta) in frutas.iter().enumerate() {
    println!("Índice: {}, Fruta: {}", indice, fruta);
    }
    }
    
    //Exercício 9: Concatenando Dois Vetores
    Crie dois vetores e combine-os em um único vetor.
    fn main() {
    let vetor1 = vec![1, 2, 3];
    let vetor2 = vec![4, 5, 6];
    let combinado = [&vetor1[..], &vetor2[..]].concat();
    
    println!("{:?}", combinado); // Deve imprimir [1, 2, 3, 4, 5, 6]
    }