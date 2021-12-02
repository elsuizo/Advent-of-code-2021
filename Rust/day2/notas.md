# Dia2

## Parte1

Ahora tenemos que ver como manejamos el submarino. Parece que puede tomar una
serie de comandos como `forward 1` `down 2` o `up 3`

 - `forward X` incrementa la posicion horizontal por `X` unidades
 - `down X` incrementa la profundidad por `X` unidades
 - `Up X` decrementa la profundidad por `X` unidades

El submarino parece que tiene la ruta planeada (nuestro input) debemos calcular
hacia donde vamos, por ejemplo:

```text
forward 5
down 5
forward 8
up 3
down 8
forward 2
```

Sabiendo que comenzamos con una posicion horizontal y vertical cero

Despues de seguir esas instrucciones tenemos una posicion horizontal de `15` y
una profundidad de `10` (que si las multiplicamos dan `150`)

calcular la posicion horizontal y la profundidad que tenemos Despues de seguir
las instrucciones. Que obtenemos si multiplicamos la posicion final horizontal
y la profundidad???

## Parte2

Basados en nuestros calculos el curso del submarino parece que no va hacia donde
queremos. Descubrimos el manual del submarino y vemos que el proceso es un poco
mas complicado. Ademas de seguir la posicion horizontal y la profundidad necesitamos
tambien seguir un tercer valor "apuntador" el cual tambien comienza en cero. Ahora
los comandos significan cosas totalmente distintas:

 - `down X` incrementa el "apuntador" por `X` unidades
 - `up X` decrementa el "apuntador" por `X` unidades
 - `forward X` hace dos cosas:
  - incrementa la posicion horizontal por `X` unidades
  - incrementa la profundidad por el "apuntador" * `X`
