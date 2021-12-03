# Dia3

## Parte1

Parece que el submarino esta haciendo ruidos raros y nos eligieron para hacer
un reporte del mismo

El reporte del submarino (el input) consiste en una lista de numeros binarios
los cuales si los decodificamos apropiadamente nos va a dar la info que buscamos
El primer parametro a buscar es el consumo de potencia

Necesitamos generar dos numeros binarios llamados "gamma rate" y "epsilon rate"
el consumo de potencia es la multiplicacion de esos dos numeros

El "gamma rate" se halla encontrando el bit mas comun en la correspondiente posicion
de todos los numeros, por ejemplo:

```text
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
```

Si consideramos solo el primer bit vemos que hay 5 `0` y 7 `1` entonces el bit
mas comun es el `1` y asi con todas las posiciones

entonces el "gamma rate" de ese reporte es: `10110` o 22 en decimal

El "epsilon rate" se calcula de manera similar en lugar del mas comun se usa el menos
comun entonces el "gamma rate" es `01001` o 9 en decimal.

Entonces el cosumo de potencia sera 22 * 9 = 198

Cual es el consumo de potencia del submarino???


## Parte2

Luego tenemos que verificar el "life support rating" que puede determinarse multiplicando
el "oxygen generator rating" con el "CO2 scrubber rating"


Comenzamos con toda la lista de numeros binarios y consideramos solo el primer
bit entonces:

 - Filtramos a los numeros que cumplen ese criterio de bit
 - Si solo nos queda un numero ese es el valor que estamos buscando
 - De otra manera, repetir el proceso considerando el proximo bit de la derecha

El criterio de bit depende del rating que estemos buscando:

 - "oxygen generator rating": Se determina con el valor mas cumun (`0` o `1`) en
   la posicion del bit actual y asi solo guardamos los numeros que tienen ese
   bit en esa posicion. Si `0` y `1` son igualmente comunes guardar valores con
   un 1 en la posicion que esta siendo considerada

 - "CO2 scrubber rating": Se detenmina con el valor menos comun(`0` o `1`) en la
   posicion del bit actual y solo guardamos numberos con ese bit en esa posicion.
   si `0` y `1` son igualmente comunes debemos guardar valores con un `0` en la
   posicion que esta siendo considerada
