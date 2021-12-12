# Dia8

## Parte 1

Apenas si pudimos alcanzar la seguridad de la cueva cuando la ballena golpea la
boca de la misma haciendo que colapse. Los sensores indican otra salida a esta
cueva en una profundidad mucho mas pronunciada, entonces no tenemos opcion que
presionar on

Mientras que el submarino lentamente hace su camino dentro de el sistema de
cueva nos damos cuenta que el display de cuatro digitos de siete segmentos esta
funcionando mal; este se debe haber daniado durante el escape. Podemos ponernos
en muchos problemas sin el, entonces tenemos que ver que es lo que pasa con el

Cada digito del display es renderizado togleando cualquiera de los siete
segmentos nombrados con las letras `a` a `g`

```text
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
 ```
Entonces para renderizar el `1` solo los segmentos `c` y `f` deben
utilizarse(como se ve en el grafico) el resto debe estar `off`, para renderizar
un `7` solo los segmentos `a`, `c` y `f` deben encenderse

El problema es que las seniales que controlan los segmentos han sido mezcladas
sobre cada display. El submarino todavia esta tratando de mostrar numeros
produciendo salidas sobre las seniales `a` hasta `g` pero esos cables estan
conectados con los segmentos de manera aleatoria. Peor aun, las conexiones
cable/segmento estan mezcladas separadamente para cada uno de los cuatro
displays(todos los digitos dentro de un display usan las mismas conexiones)

Entonces es posible que solo sepamos que los cables de senial `b` y `g` esten
encendidos, pero esto no quiere decir que los segmentos `b` y `g` hallan sidos
encendidos: el unico digito que usa dos segmentos es `1`, por lo que debe
significar que los segmentos `c` y `f` deben estar encendidos. Con solo esa
informacion todavia no podemos decir cual cable (`b`/`g`) va a cual segmento
`c`/`f`. Para esto necesitamos colectar mas informacion

Para cada display miramos las seniales que cambian por un tiempo y anotamos
todos los 10 patrones unicos de seniales que vemos y entonces escribe un solo
valor de salida de cuatro digitos(el input del problema). Usando los patrones de
seniales debemos poder descifrar que patron corresponde a cual digito.

Por ejemplo, aca vemos una sola linea de nuestra notas:

```text
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
cdfeb fcadb cdfeb cdbaf
```

Cada entrada consiste en `10` patrones *unicos* de seniales el `|` es un
delimitador y finalmente los cuatro valores de salida
Dentro de una entrada, se utilizan las mismas conexiones de cable / segmento
(pero no sabe cuáles son realmente las conexiones)

Estos patrones unicos de seniales corresponden a `10` maneras diferentes en las
que el submarino trata de renderizar un digito usando la conexion
actual cable/segmento. Porque `7` es el unico digito que usa tres segmentos,
`dab` en el ejemplo anterior significa que para renderizar un `7` las lineas de
senial `d`, `a` y `b` estan encendidas. Porque `4` es el unico digito que usa
tres segmentos cuatro segmentos `eafb` significa que para renderizar un `4` las
lineas de seniales `e`, `a`, `f` y `b` estan encendidas

Usando esta informacion, deberiamos poder sacar las combinaciones de
seniales/cables que corresponden a cada uno de los 10 digitos. Entonces podemos
decodificar los cutro valores de salida. Desafortunadamente en el ejemplo
anterior todos los digitos en los valores de salida (`cdfeb` `fcadb` `cdfeb`
`cdbaf`) usan cinco segmentos y son mas dificultosos de deducir.

Por ahora nos enfocamos en los digitos faciles. Consideremos un ejemplo mas
largo:

```text
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce
```

Porque los digitos `1`, `4`, `7` y `8` cada uno usa un numero unico de
segmentos, podremos deducir cual combinacion de seniales corresponde a esos
digitos. Contando solo los digitos en las seniales de salida(la parte despues de
`|` sobre cada linea) en el ejemplo anterior hay `26` instancias de digitos que
usan un numero unico de segmentos (valores resaltados)

En los valores de salida, cuantas veces los digitos `1`, `4`, `7` o `8`
aparecen???
