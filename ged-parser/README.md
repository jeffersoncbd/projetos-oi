# GED PARSER

Uma aplicação escrita em [rust](https://www.rust-lang.org/) para gerar o relatório de faturas usando como base o arquivo CSV do relatório (exclusivo para o fatura resumida whatsapp/email).

---
## Instalação
- Para executar esta aplicação será necessário instalar o rust como mostrado [aqui](https://www.rust-lang.org/tools/install) (não importa qual OS).
- Entre no diretório do projeto e execute `cargo build` para instalar todas as dependências.

---
## Compilar
O projeto pode ser executado diretamente utilizando `cargo run`, mas o ideal é compilar o binário que é gerado com otimizações (com finalidade de uso em produção). Para isso basta executar `cargo build --release` (tanto no Windows quanto no Linux) que o binário será compilado e salvo em `target/release/` sendo gerado um `ged-parser` no Linux e `ged-parser.exe` no Windows. O arquivo compilado pode ser copiado/movido para qualquer pasta do SO que irá funcionar normalmente.

### Compilar para "Oracle Linux Server"
É possível gerar um binário para servidores utilizando uma outra máquina (`cargo build --release` não irá funcionar aqui pois por padrão o rust gera binários que utilizam o `GLIBC 2.17` ou superior, mas máquinas como a `gedpx03a` utilizam `GLIBC 2.12` se isso mudar um dia, será possível verificar a versão atual do `GLIBC` com o comando `ldd --version`). Para isso deverá ser instalado a versão de destino no sistema com o comando:
```bash
rustup target add [VERSÃO]
```
No caso a versão da `gedpx03a` é `x86_64-unknown-linux-gnu`. Depois será necessário instalar o ziglang, é mais simples a instalação utilizando o pip do python3:
```bash
pip3 install ziglang
```
Mas caso não possua nem queira o python3 é possível instalar usando as instruções da [documentação do ziglang](https://ziglang.org/learn/getting-started/#installing-zig). Agora deve ser instalado a integração entre o rust e o ziglang:
```bash
cargo install cargo-zigbuild
```
Por fim gerar o binário com esta estrutura de comando:
```bash
cargo zigbuild --target [OS VERSION].[GLIBC VERSION] --release
```
Para o servidor `gedpx03a` o comando ficaria assim:
```bash
cargo zigbuild --target x86_64-unknown-linux-gnu.2.12 --release
```
Isso irá gerar um binário em `target/x86_64-unknown-linux-gnu/release/` com nome `ged-parser` que poderá ser transferido para o servidor via `sftp` e executado normalmente.

_OBS: Para tornar o binário executável no servidor, talvez seja necessário executar primeiro:_
```bash
chmod +x ./ged-parser
```

---
## Execução
Para rodar o programa execute passando os parâmetros necessários sendo:
1. Path para o CSV com os dados.
2. Filtro dos dados (whatsapp ou email).

### Windows
```bat
.\ged-parser.exe relatorio.csv whatsapp
```

### Linux
```bash
./ged-parser relatorio.csv email
```

Ao executar o binário passando os parâmetros necessários o programa irá construir um arquivo `.png` de acordo com o filtro: `whatsapp.png` ou `email.png`.

