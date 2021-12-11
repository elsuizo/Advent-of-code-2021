# Dia6

## Parte1

El fondo del mar se esta volviendo cada vez mas enpinado, quizas las llaves del
trineo se perdieron de esta manera, un banco de peces linterna pasa nadando. Ellos
deben expandirse de manera rapida para llegar a esa cantidad de numbero (capaz que
exponencialmente) debemos modelar la tasa de nacimiento para estar seguros

Aunque no sabemos nada sobre esta especie en especial nosotros hicimos unas estimaciones
sobre sus atributos. Seguramente cada pez linterna crea uno nuevo una vez cada 7
dias

Sin embargo este proceso no esta necesariamente sincronizado entre cada pez linterna
un pez linterna debe tener 2 dias de descanso hasta que pueda crear otro pez, mientras
que otro puede que necesite 4 dias. Entonces podemos modela a cada pez como un solo numero
que representa el numero de dias hasta que crea un nuevo pez linterna.

Es mas, razonamos que un nuevo pez linterna puede necesariamente necesitar mas tiempo
antes de que pueda producir mas peces: dos mas dias para su primer ciclo

Entonces, supongamos que tenemos un pez linterna con reloj interno de 3

 - Despues de 1 dia, su reloj interno se convierte en 2
 - Despues de otro dia su reloj interno se convierte en 1
 - Despues de otro dia su reloj interno se convierte en 0
 - Despues de otro dia su reloj interno se resetea a 6 y este puede crear un nuevo
   pez linterna con un reloj interno de 8
 - Despues de otro dia, el primer pez debe tener un reloj interno de 5 y el segundo
   pez linterna debe tener un reloj interno de 7

Un pez linterna que puede crear un nuevo pez resetea su reloj a 6, no a 7 (porque el 0 esta incluido
como valor valido de timer). El nuevo pez linterna comienza con un reloj interno de 8
y no comienza a contar hasta el proximo dia.

Viendo lo que queremos hacer el submarino automaticamente produce una lista de las
edades de muchos de los peces que estan cerca(nuestro input). Por ejemplo supongamos
que nos dan la siguiente lista:

```text
3,4,3,1,2
```

Esta lista significa que el primer pez tiene un reloj interno de 3, el segundo de 4
el tercero de 3 y asi hasta el ultimo que tiene un reloj interno de 2. Simulando
estos peces sobre algunos dias nos da lo siguiente:

```text
Initial state: 3,4,3,1,2
After  1 day:  2,3,2,0,1
After  2 days: 1,2,1,6,0,8
After  3 days: 0,1,0,5,6,7,8
After  4 days: 6,0,6,4,5,6,7,8,8
After  5 days: 5,6,5,3,4,5,6,7,7,8
After  6 days: 4,5,4,2,3,4,5,6,6,7
After  7 days: 3,4,3,1,2,3,4,5,5,6
After  8 days: 2,3,2,0,1,2,3,4,4,5
After  9 days: 1,2,1,6,0,1,2,3,3,4,8
After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
```

Cada dia un 0 se convierte en 6 y agrega un 8 al final de la lista, mientras que
los otros numeros se decrementan en 1 si estuvo presente al comienzo del dia

En este ejemplo, despues de 18 dias hay un total de 26 peces. Despues de 80 dias
habra un total de 5934

Hay que encontrar la manera de simular a los peces linterna. Cuantos peces hay despues
de 80 dias???
