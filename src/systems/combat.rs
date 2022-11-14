use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // provides a list of entities that wish to initiate combat
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    // specifies that targets are a tuple of Entity, Entity
    // rust generic turbofish function::<Type>()
    let target : Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        // map the iterator and translate it
        // takes entity and attack as inputs, returns tuple containing entity and attack target
        // have to dereference entity because iterating over it turned it into a ref
        .map(|(entity, attack)| (*entity, attack.target))
        // take iterator's results and put them in a vector
        .collect();

    target.iter().for_each(|(message, target)| {
        // if let activates only if target has health
        if let Ok(mut health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                // if target health is less than 1 it removes it
                commands.remove(*target);
            }
            println!("Health after attack: {}", health.current);
        }
        // deletes want to attack message
        commands.remove(*message);
    })
}