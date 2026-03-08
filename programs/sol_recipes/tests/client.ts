// Client
// obtener el PDA
const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("recetario"), pg.wallet.publicKey.toBuffer()],
  pg.program.programId
);

async function check() {
  try {

    //traer cuenta
    const data = await pg.program.account.recetario.fetch(pda);

    console.log("Mis recetas guardadas:");
    console.log("Creador:", data.owner.toString());
    console.log("Nombre del Recetario:", data.nombreRecetario);

    data.recetas.forEach((receta, i) => {

      console.log("\nReceta " + (i + 1));
      console.log("Nombre:", receta.nombre);
      console.log("Ingredientes:", receta.ingredientes);

      if (receta.probada) {
        console.log("Estado: probado");
      } else {
        console.log("Estado: pendiente");
      }

      const fecha = new Date(receta.fechaCreacion.toNumber() * 1000);
      console.log("Fecha:", fecha.toLocaleDateString(), fecha.toLocaleTimeString());

    });

  } catch (error) {
    console.log("No se encontrarón recetas.");
  }
}

check();
