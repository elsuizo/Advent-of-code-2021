#Dia9

## Parte1

Estas cuevas parece que tienen tubos de lava. Algunas partes parece que son todavia
volcanicamente activas; un pequeño agujero en el techo lentamente libera humo como
si fuera una lluvia. Si podemos modelar como este humo cae podremos evitarlo. El submarino
genera un mapa de alturas del piso cerca de nosotros (el input)

El humo va hacia los puntos donde hay una menor area. Por ejemplo consideremos

```text
2199943210
3987894921
9856789892
8767896789
9899965678
```

Cada numero corresponde a la altura de una locacion en particular, donde 9 es la
mas alta y 0 es la mas baja que pueden haber

Nuestro primer objetivo es encontrar los puntos mas bajos(las locaciones que son
mas bajas que cualquier adyacente locacion. La mayoria de las locaciones tienen
cuatro locaciones adyacentes (up, down, left y right)) las locaciones sobre el borde
o en una esquina tienen tres o dos locaciones adyacentes, respectivamente. Las locaciones
en diagonal no cuenta como adyacentes

En el ejemplo de arriba hay 4 puntos minimos que estan resaltados; dos en la primer
columna (1 y 0) uno en la tercer columna (un 5) y uno en la ultima fila que es el
5 tambien. Todas las otras locaciones sobre el mapa de alturas tienen alguna locacion
adyacente que es menor y por eso no son puntos minimos

El nivel de riesgo de un punto minimo es 1 mas su altura. En el ejemplo anterior
el riesgo de nivel de los puntos son 2, 1, 6 y 6. La suma de los riesgos de nivel
de todos los puntos minimos en el mapa de alturas es entonces 15

Hallar todos los puntos minimos sobre nuestro mapa de alturas. Cual es la suma de
los riesgos de nivel de todos los puntos sobre nuestro mapa de alturas
