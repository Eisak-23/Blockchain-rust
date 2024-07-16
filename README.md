##Test How to create a  Blockchain

<br>

#Resumen del Proyecto

- Estructura del Código

	El proyecto está dividido en dos partes principales: la definición y operaciones de los 	bloques (Block) y la implementación de la cadena de bloques (Blockchain).

#####Block (Bloque):

- **Struct Block**: Representa un bloque en la cadena. Contiene campos como 				**timestamp, transactions, prev\_hash, hash, height **y** nonce.**

- Métodos clave:

- **new\_genesiss():** Crea el bloque génesis (bloque inicial).

- **new\_block():** Crea un nuevo bloque con datos específicos.

- **proof\_of\_work()**: Implementa el algoritmo de Prueba de Trabajo para encontrar un hash válido.

- **validate()**: Verifica si el hash del bloque cumple con la dificultad requerida.

- **prepare\_hash():** Prepara los datos del bloque para ser hasheados.

**Blockchain:**
<br>

- **struct Blockchain: **Representa la cadena de bloques. Utiliza la base de datos sled para almacenar los bloques.

- Métodos clave:

- **new(): **nicializa una nueva cadena de bloques. Si ya existe una cadena en la base de datos, la carga; de lo contrario, crea un bloque génesis.

- **add\_block():** Agrega un nuevo bloque a la cadena después de validar y calcular su hash.

- **iter():** Itera sobre los bloques de la cadena.

- **Iterador Blockchain:**

- **struct BlockchainIter**: Itera sobre los bloques 	de la cadena de bloques.

	Implementa el trait Iterator para poder recorrer 	secuencialmente los bloques.

	**Pruebas Unitarias:**

	- Se incluyen pruebas básicas (test\_blockchain()) que verifican la creación y la iteración correcta de la cadena de bloques, así como la impresión de los datos de cada bloque.
	<br>
	<br>

**Funcionamiento**

- **Inicialización**: Al inicializar Blockchain, se abre una base de datos sled. Si existe un último bloque registrado, se carga como current\_hash. De lo contrario, se crea un bloque génesis y se guarda en la base de datos.

- **Añadir Bloques:** Para añadir un bloque, se calcula su hash y se almacena en la base de datos, actualizando también el último hash.

- **Iteración: **El iterador BlockchainIter permite recorrer todos los bloques de la cadena, comenzando desde el último bloque (current\_hash) y retrocediendo hacia el génesis.

**Conclusiones**
<br>

	Este proyecto proporciona una base sólida para entender cómo implementar 
 	una blockchain simple en Rust, utilizando conceptos como la estructura de datos de bloques, el algoritmo de Prueba de Trabajo para asegurar la cadena,
  	y una base de datos para almacenarla de manera persistente. Se pueden agregar más características y mejoras según sea necesario para aplicaciones específicas de blockchain.


<br>
