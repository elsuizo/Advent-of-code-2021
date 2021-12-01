# Dia1

## Parte 1
Estamos en un negocio personal en un submarino(jajsjasj) y parece que sono la alarma
y cuando fuimos a ver a un elfo se le ha caido la llaves del trineo al mar, pero
parece que esta preparado por si algo asi sucedia... ya que tiene una antena que
emite una senial que podemos seguir su potencia, para ello hay un display que tiene
esa potencia en forma de estrellas de 0-50
Nuestro instinto nos dice que debemos recolectar esas 50 estrellas antes del 25 de
diciembre

Mientras el submarino baja hacia las profundidades del oceano automaticamente emite
un eco de sonar para saber donde esta el piso del mar, tenemos un display que nos
dice esa medida (el input del problema) cada linea es una medida de la profundidad
del mar, por ejemplo supongamos que tenemos los siguientes datos:

```text
199
200
208
210
200
207
240
269
260
263
```

Lo primero que debemos hacer es computar cuan rapido la profundidad esta creciendo
Para hacer esto debemos contar la cantidad de veces que la medicion de profundidad
crece desde la ultima medicion. Para el ejemplo anterior tenemos:

```text
199 (N/A - no previous measurement)
200 (increased)
208 (increased)
210 (increased)
200 (decreased)
207 (increased)
240 (increased)
269 (increased)
260 (decreased)
263 (increased)
```

Hay 7 crecimientos. Cuantas mediciones son mas grandes que las anteriores en nuestro
input???

## Parte 2

Ahora lo que tenemos que hacer es comparar las sumas de las ventanas pero de 3
elementos y ver cuando aumenta o no
