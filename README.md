# Funcionamento

Rode o projeto usando
```
$ cargo run
```

Este projeto apresenta 5 tipos diferentes de objetos, tentando criar uma cena de academia:
halter(3d);
peso do aparelho de costas (lat pulldown)(3d);
barra do aparelho de costas(3d);
banquinho (3d);
pessoa (uma em 2d e outra 3d);

O halter pode ser expandido ou contraído por uma transformação de escala pressionando as teclas "X" e "N" respectivamente.

Uma pessoa no lado esquerdo da tela pode flexionar o braço através de uma transformação de rotação ao redor do eixo z pressionando as teclas C (flexiona) e R (relaxa).

Uma pessoa do lado direito da tela realiza um puxador na máquina através de uma translação. Ele puxa quando é pressionada a tecla "L" e solta quando pressionada a tecla "H".

Podem ser exibidas e ocultadas as malhas poligonais dos objetos pressionando a tecla "P".

# Arquivos
O projeto está separado em três arquivos diferentes dentro do diretório "src/":

## alglin.rs
   Implementa matrizes, vetores e operações que podem ser realizadas em matrizes e vetores em rust.

## objetos.rs
   Utiliza noções de geometria analítica para gerar os vértices dos objetos básicos da cena, sem desenhar ou posicionar eles.

## main.rs
   Realiza operações do pipeline gráfico, desenha os vértices, cria uma janela para o usuário visualizar e interagir com a cena e posiciona os objetos.
    


