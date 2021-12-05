# Dia5

## Parte1

Parece que nos topamos con nubes hidrotermales que producen nubes que son peligrosas
y lo que queremos es evadirlas, estas parece que forman lineas, el submarino por
suerte produce una lista de estas lineas de nubes (el input) para que las revisemos
por ejemplo:

```text
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
```

Cada linea es dada como un segmento de linea en el formato: `x1,y1 -> x2,y2` donde
`x1,y1` son las coordenadas de uno de los finales de la linea y `x2,y2` son las
coordenadas del otro final. Estas lineas incluyen los puntos al final de las mismas
en ambos extremos. En otras palabras:

 - Una entrada como `1,1 -> 1,3` cubre a los puntos `1,1`, `1,2` y `1,3`
 - Una entrada como `9,7` -> `7,7` cubre a los puntos `9,7`, `8,7` y `7,7`

Por ahora solo vamos a considerar lineas horizontales y verticales: lineas donde
`x1=x2` o `y1=y2`

Entonces las lineas horizontales y verticales de la lista anterior producen el siguiente
diagrama:

```text
.......1..
..1....1..
..1....1..
.......1..
.112111211
..........
..........
..........
..........
222111....
```

En este diagrama la punta de arriba a la izquierda es `0,0` y la punta de abajo
a la derecha es `9,9`. Cada posicion se muestra como el numero de lineas que cubren
ese punto(la cantidad de lineas que pasan por ahi) o un `.` cuando ninguna lo atraviesa

Para evitar las areas mas peligrosas necesitamos determinar el numero de puntos
donde al menos dos lineas se cruzan. En el ejemplo anterior es cualquier lugar
donde hay un 2 o un numero mas grande, si los contamos son 5 puntos.

Considerando solo lineas horizontales y verticales. Cuantos puntos tienen dos lineas
o mas que pasan por el???


## Parte2

Desafortunadamente considerar solo las lineas horizontales y verticales no nos da
una buena lectura de la situacion, por ello necesitamos considerar tambien a las
lineas diagonales, por las limitaciones del sistema de mapeo de venteo las lineas
en nuestra lista van a ser horizontales, verticales y diagonales con una pendiente
de 45 grados. En otras palabras:

 - Una entrada como `1,1 -> 3,3` cubre los puntos `1,1`, `2,2` y `3,3`
 - Una entrada como `9,7 -> 7,9` cubre los puntos `9,7`, `8,8` y `7,9`

Tenemos que determinar el numero de puntos donde al menos dos lineas se superponen
