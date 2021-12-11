# Dia7

## Parte1

Una ballena gigante ha decidido que nuestro submarino va a ser su proxima comida
y es mucho mas rapida que nosotros. No hay lugar a donde correr!!!

De repente un ejambre de cangrejos(cada uno en un pequeño submarino) nos viene a
rescatar quieren hacer un agujero en el piso del oceano; indicado como un

Los submarinos de los cangrejos lo unico que necesitan para estar alineados
antes de que tener el suficiente poder para hacer un agujero en el piso. Sin
embargo parece que no van a poder alinearse antes de ser atrapados por la
ballena, capaz que podemos ayudar!!!

Hay una sola trampa los submarinos de los cangrejos solo pueden moverse
horizontalmente

Nosotros rapidamente hacemos una lista de las posiciones horizontales de cada
cangrejo (nuestro input). El combustible de los submarinos es limitado, entonces
necesitamos encontrar la manera de hacer que la posicion horizontal de todos
matchee mientras que gasten la menor cantidad de combustible posible.

Por ejemplo, consideremos las siguientes posiciones horizontales:

```text
16,1,2,0,4,2,7,1,2,14
```

Esto significa que hay un cangrejo con una posicion horizontal `16`, un cangrejo
con una posicion horizontal de `1` y asi sucesivamente...

Cada cambio de 1 paso en la posicion horizontal de un solo cangrejo cuesta 1
unidad de combustible

Tu puedes elegir cualquier posicion horizontal para alinearlos a ellos, pero
solo una es la que consume el menor combustible en posicion horizontal y es la
posicion `2`

 - Moverse desde `16` a `2`: `14` unidades de combustible
 - Moverse desde `1` a `2`: 1 unidades de combustible
 - Moverse desde `2` a `2`: 0 unidades de combustible
 - Moverse desde `0` a `2`: 2 unidades de combustible
 - Moverse desde `4` a `2`: 2 unidades de combustible
 - Moverse desde `2` a `2`: 0 unidades de combustible
 - Moverse desde `7` a `2`: 5 unidades de combustible
 - Moverse desde `1` a `2`: 1 unidades de combustible
 - Moverse desde `2` a `2`: 0 unidades de combustible
 - Moverse desde `14` a `2`: 12 unidades de combustible

La suma de todo el combustible gastado es `37` que es la minima posible; salidas
mas costosas incluyen alinearse en la posision `1` (`41` de combustible)
posicion `3` (`39` de combustible) o posicion `10` (`71` de combustible)

Determinar la posicion horizontal que los cangrejos pueden alinearse para usar
el menor combustible posible. Cuanto combustible debe gastar para alinearse en
esa posicion

## Parte2

Los cangrejos parece que no estan interesados en nuestra propuesta. Parece que
no entendimos la ingenieria de los cangrejos

Parece que los motores de los submarinos no gastan el combustible a una tasa
constante, en lugar cada cambio de 1 paso en la posicion horizontal cuesta 1
unidad de combustible mas que la anterior: el primer paso cuesta `1` el segundo
paso cuesta `2`, el tercer paso cuesta `3` y asi sucesivamente.

Cuando cada cangrejo se mueve, moverse mas se convierte mas costoso. Esto cambia
la que era la mejor posicion horizontal para alinearse; en el ejemplo anterior,
esto se convierte en `5`

 - Moverse desde `16` a `5`: `66` de combustible
 - Moverse desde `1` a `5`: `10` de combustible
 - Moverse desde `2` a `5`: `6` de combustible
 - Moverse desde `0` a `5`: `15` de combustible
 - Moverse desde `4` a `5`: `1` de combustible
 - Moverse desde `2` a `5`: `6` de combustible
 - Moverse desde `7` a `5`: `3` de combustible
 - Moverse desde `1` a `5`: `10` de combustible
 - Moverse desde `2` a `5`: `6` de combustible
 - Moverse desde `14` a `5`: `45` de combustible

Esto tiene un costo total de 168 unidades de combustible. Esto es la nueva
posible salida mas barata que podemos tener, la vieja posicion que teniamos `2`
ahora cuesta `206` unidades de combustible

Determinar la posicion horizontal que los cangrejos pueden alinearse para usar
la menor cantidad de combustible posible para que podamos hacer la ruta de
escape. Cuanto combustible deben gastar para alinearse en esa posicion ???
