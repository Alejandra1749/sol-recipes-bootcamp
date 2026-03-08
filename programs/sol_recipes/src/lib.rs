use anchor_lang::prelude::*;

declare_id!("C7Vbv4c6j2eYYa8cnFi8K1ZirSmKjEErX78T8GjKdHc2");

#[program]
pub mod sol_recipes {
    use super::*;

    pub fn crear_recetario(ctx: Context<NuevoRecetario>, nombre: String) -> Result<()> {
        let recetario = &mut ctx.accounts.recetario;
        recetario.owner = ctx.accounts.owner.key();
        recetario.nombre_recetario = nombre;
        recetario.recetas = Vec::new();
        Ok(())
    }

    pub fn agregar_receta(
        ctx: Context<ActualizarRecetario>,
        nombre: String,
        ingredientes: String,
    ) -> Result<()> {
        let fecha = Clock::get()?;
        let nueva_receta = Receta {
            nombre,
            ingredientes,
            probada: false,
            fecha_creacion: fecha.unix_timestamp,
        };

        ctx.accounts.recetario.recetas.push(nueva_receta);
        Ok(())
    }

    pub fn alterar_recetario(ctx: Context<ActualizarRecetario>, nombre: String) -> Result<()> {
        let recetario = &mut ctx.accounts.recetario;

        for receta in recetario.recetas.iter_mut() {
            if receta.nombre == nombre {
                receta.probada = !receta.probada;
                return Ok(());
            }
        }
        err!(ErroresCocina::RecetaNoEncontrada)
    }

    pub fn eliminar_receta(ctx: Context<ActualizarRecetario>, nombre: String) -> Result<()> {
        let recetario = &mut ctx.accounts.recetario;
        let posicion = recetario.recetas.iter().position(|r| r.nombre == nombre);

        if let Some(index) = posicion {
            recetario.recetas.remove(index);
            Ok(())
        } else {
            err!(ErroresCocina::RecetaNoEncontrada)
        }
    }
    pub fn ver_recetas(ctx: Context<ActualizarRecetario>) -> Result<()> {
        let recetario = &ctx.accounts.recetario;
        msg!("--- Recetario <*_*> ---");
        msg!("Creador: {:?}", recetario.owner);
        msg!("Recetas actuales: {}", recetario.recetas.len());
        for receta in recetario.recetas.iter() {
            msg!(
                "Platillo: {} | Ingredientes: {} | Estado: {} | Fecha en la que fue creada: {}",
                receta.nombre,
                receta.ingredientes,
                receta.probada,
                receta.fecha_creacion
            );
        }
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Recetario {
    pub owner: Pubkey,
    #[max_len(50)]
    pub nombre_recetario: String,
    #[max_len(10)]
    pub recetas: Vec<Receta>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Receta {
    #[max_len(50)]
    pub nombre: String,
    #[max_len(200)]
    pub ingredientes: String,
    pub probada: bool,
    pub fecha_creacion: i64,
}

#[derive(Accounts)]
pub struct NuevoRecetario<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + Recetario::INIT_SPACE,
        seeds = [b"recetario", owner.key().as_ref()],
        bump
    )]
    pub recetario: Account<'info, Recetario>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarRecetario<'info> {
    #[account(mut, has_one = owner)]
    pub recetario: Account<'info, Recetario>,
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ErroresCocina {
    #[msg("La receta no existe.")]
    RecetaNoEncontrada,
}
