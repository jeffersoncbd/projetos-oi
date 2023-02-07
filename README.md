# projetos-oi
> Projetos desenvolvidos para a Oi

Legenda das pastas:
### BOT
Este projeto na verdade é um CLI que consome a lib [tg-api](https://crates.io/crates/tg-api) (tambem desenvolvida por mim) para conseguir enviar mensagens/imagens/arquivos/outputs usando o BOT do Telegram @fatura_oi_bot.
```bash
tg USER_ID MESSAGE
tg -i USER_ID /path/to/image
etc...
```

### Chain monitor
Projeto ainda em desenvolvimento, mas que usa o bin do projeto anterior (CLI do Bot do telegram) para notificar via Telegram o inicio/fim de JOBs do Control M, no momento ele consegue observar somente jobs que registram no banco de dados do Control, mas há formas a serem implementadas para conseguir rastrear qualquer job.
```
chain-monitor -i NOME_DO_JOB
chain-monitor -f NOME_DO_JOB
```
> O JOB NOME_DO_JOB teve INICIO em 01/01/2000 às 00:00.

### GED Parser
Este projeto está sendo desfeito, ele começou a ter responsabilidade demais e está sendo dividido em três outros projetos

### Invoices Status
Antigo `GED Parser`, este é o relatório `Verificação E-mail Seguro` que verifica situações das faturas da Oi, antes este relatório era feito manualmente nessas etapas:
1. Execução de query SQL no banco
2. Spool de dados em um .csv
3. Criava-se uma planilha dinâmica para resumir os dados
4. Era tirado 2 prints sendo um com os dados de `E-mail` e outro com dados de `Whatsapp`
5. Enviava-se os prints para o grupo de gerentes do Whatsapp.
Agora o processo é 100% autonomo graças à duas libs que desenvolvi:
- [image-builder](https://crates.io/crates/image-builder): possibilita escrever textos num .png usando Rust.
- [spreadsheet-maker](https://crates.io/crates/spreadsheet-maker): uma simulação de Excel sem interface para Rust que utiliza o `image-builder` para gerar .png da planilha criada ou simplesmente exportar um .csv.
Depois de gerado os .png o projeto envia eles utilizando a lib [tg-api](https://crates.io/crates/tg-api).

### SCAR
O SCAR é um projeto que foi criado pelo Júlio (antecessor a mim), mas que exigia do usuário um conhecimento prévio de Shell para operar... foi criado uma espécie de "PSEUDO LINGUAGEM" para que fosse possível configurar o sistema sem a necessidade de conhecimentos técnicos. Tambem foi tratado alguns erros e reexposto os outputs do projeto, pois antes tudo era jogado fora (/dev/null), com isso alguns bugs foram encontrados e corrigidos.

### Shells
Nesta pasta não há projetos, só um pequeno script para verificação do link da FAST, mas que tambem faz uso do Telegram para notificar caso este link esteja fora do ar.
