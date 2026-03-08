SolRecipes - Recetario en Solana
  SolRecipes es un pequeño programa desarrollado en Rust usando Anchor que permite guardar recetas de cocina en la blockchain de Solana.
  La idea del proyecto es que cada usuario tenga su propio recetario asociado a su wallet y pueda administrar sus recetas.
  
  ¿Qué hace el proyecto?
  
  El programa permite manejar recetas de cocina dentro de la blockchain. Básicamente funciona como un pequeño sistema donde el usuario puede:
  
    -Crear su propio recetario asociado a su wallet.
    
    -Agregar nuevas recetas con su nombre e ingredientes.
    
    -Guardar automáticamente la fecha en que se creó la receta usando el reloj de Solana.
    
    -Marcar una receta como probada o pendiente.
    
    -Eliminar recetas si ya no las necesita.
  
  Cada recetario se guarda en una PDA , por lo que cada usuario tiene su propio espacio de datos y solo el dueño de la wallet puede modificarlo.
  
  Arquitectura
  
  Cada usuario tiene un recetario asociado a su wallet:
  
    Owner (Wallet)
       |
       └── Recetario (PDA)
            ├── Receta 1
            ├── Receta 2
            └── Receta 3
    Estructuras principales
    Recetario
  
  Guarda la información principal del usuario.
  
    -owner: wallet del dueño del recetario
    
    -nombre_recetario: nombre que el usuario le da a su recetario
    
    -recetas: lista donde se almacenan las recetas
  
  Receta
  
  Cada receta guarda la siguiente información:
  
    -nombre: nombre del platillo
    
    -ingredientes: lista de ingredientes
    
    -probada: indica si la receta ya fue probada
    
    -fecha_creacion: fecha en la que se agregó la receta
  
  Funciones del programa
  El programa tiene varias instrucciones que permiten manejar el recetario:
  
    -crear_recetario(nombre) :Crea la cuenta del recetario y la vincula con la wallet del usuario.
    
    -agregar_receta(nombre, ingredientes) :Agrega una nueva receta al recetario.
    
    -alterar_recetario(nombre) :Cambia el estado de una receta (probada o pendiente).
    
    -eliminar_receta(nombre) :Elimina una receta específica.
    
    -ver_recetas(): Muestra en los logs la información de todas las recetas guardadas.
  
  PDA utilizada
    El recetario se genera usando estas semillas:
    ["recetario", owner_pubkey]
    Esto permite que cada usuario tenga una dirección única para su recetario.
  
  Ejemplo de uso
  
    crear_recetario("Cocina de Alejandra")
    
    agregar_receta("Pasta Bolognesa", "Carne, tomate, pasta")
    
    alterar_recetario("Pasta Bolognesa")
    
    ver_recetas()
    
    Esto mostrará en los logs las recetas guardadas junto con su fecha de creación.
  
  Tecnologías usadas
  
    -Solana
    
    -Anchor Framework
    
    -Rust
    
    -TypeScript (para pruebas)
  
  Proyecto realizado por Pamela Alejandra Hernández Martínez.
