# Dia4

## Parte1

Estamos casi a 1.5 kms abajo de la superficie del mar, es tan profundo que no podemos
ver siquiera la luz del sol, lo que si podemos ver es un calamar gigante atado
al submarino. Capaz que quiere jugar al bingo???

bingo se juega sobre un conjunto de cartas que consisten en una grilla de numeros
de 5x5, los numeros se elijen al azar y los numeros elejidos se marcan en todas
las cartas que aparecen. (los numeros no aparecen en todas las cartas) Si todos
los numeros de una fila o una columna estan marcados esa carta es la ganadora(las
diagonales no cuentan)

El submarino tiene un sistema de bingo para ayudar a los pasajeros (ahora nosotros
y el calamar) a pasar el tiempo. El genera automaticamente una orden random en la
cual se dibujan los numeros y un conjunto de cartas (nuestro input) por ejemplo:

```text
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
```

Despues de que los primeros cinco numeros son dibujados (7, 4, 9, 5, y 11) no hay
ganadores, pero las cartas son marcadas como sigue:

(se marcan los numeros en las cartas)

Despues de los siguientes 6 numeros (17, 23, 2, 0, 14 y 21) todavia no hay ganadores

Finalmente 24 es marcado y hay una fila que se ha completado en la tercera carta

14, 21, 17, 24, 4

El puntaje de esa carta se calcula. Primeos se suman todos los numeros que no estan
marcados en esa carta en este caso la suma es 188. Luego se multiplica esa suma por
el numero que recien ha salido en este caso el 24, osea 188 * 24 = 4512

Para garantizar la victoria contra el calamar hay que ver cual carta va a ganar
primero. Cual sera nuesto puntaje final si elejimos esa carta???

## Parte2

Tenemos que ver ahora cual es la carta que es la ultima en ganar
