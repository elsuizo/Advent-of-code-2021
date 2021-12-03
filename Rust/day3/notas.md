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
