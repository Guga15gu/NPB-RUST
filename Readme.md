# NPB in Rust
Implementação de alguns benchmarks da Nasa https://www.nas.nasa.gov/software/npb.html <br>

# Build and Run
Para compilar um arquivo específico existe um arquivo Makefile, é necessário possuir o cargo, o make e o compilador Rust. Os executáveis ficam na pasta `/target/release/` <br>

### Para compilar
`make <nome-bench> CLASS=<classe>`

Exemplos:

`make is-ser CLASS=B`

`make ep-ser CLASS=S`

`make ep-parallel CLASS=W`

### Para executar:

`./target/release/<nome-bench>`

Exemplos:

`./target/release/ep-ser`

`./target/release/ep-parallel`

`./target/release/is-ser`
