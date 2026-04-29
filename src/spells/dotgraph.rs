
use crate::{
    mods::{
        mod_descriptor_asset::ModDescriptorAsset, mod_descriptor_loaded_assets::ModDescriptorLoadedAssets
    },
    spells::{
        executor::Spell,
        spell_component_asset::SpellComponentAsset,
    },
};
use bevy::
    asset::{Assets, Handle}
;
use petgraph::{Graph, dot::dot_parser::ParseFromDot};
use petgraph::
    dot::dot_parser::{DotAttrList, DotNodeWeight}
;

#[derive(Clone, Debug)]
pub struct SpellGraphNode {
    pub descriptor: Handle<SpellComponentAsset>,
}

#[derive(Clone, Debug)]
pub struct SpellGraphTransition {
    pub modifier_event: Option<String>,
}

pub fn dot_graph_to_spell_graph(
    input: &str,
    current_mod_name: &str,
    descriptors: &ModDescriptorLoadedAssets,
    assets: &Assets<ModDescriptorAsset>,
) -> Result<Spell, String> {
    let dot_graph = parse_raw_graph(input)?;
    // nodes have id's and "label" attributes we interpret as comma separated mod name and spell name
    // if no mod name is provided we assume the current mod name
    // edges have labels too which specify modifiers
    // we expect a single node to be called Start which we will treat as the root and parse from there

    // resolve spells 
    let mut last_err: Result<_, String> = Ok(());
    let mut start_node = None;
    let resolved_graph = dot_graph.map_owned(|i, node| {
        if node.id == "Start" {
            start_node = Some(i);
        }

        let res: Result<Handle<SpellComponentAsset>, String> = (|| {

            let label = node.attr.elems.iter()
                .find_map(|(key,val)| (*key == "label").then_some(*val))
                .ok_or_else(|| format!("Node: '{}' missing label with mod and spell component name", node.id))?;
            // remove quotes
            let label = label.trim_matches('\"');

            let (mod_name, spell_component_name) = label.split_once(",")
                .unwrap_or((current_mod_name, label));
            
            let descriptor = descriptors
                .get_spell_component_by_name(mod_name, spell_component_name, assets)
                .ok_or_else(|| format!("Node: '{}', mod: '{mod_name}', spell component: '{spell_component_name}', was not found.", node.id))?;
            Ok(descriptor)
        })();
        let descriptor = match res {
            Ok(descriptor) => descriptor,
            Err(e) => {
                last_err = Err(e);
                return SpellGraphNode{
                    descriptor: Handle::default()
                }
            },
        };

        SpellGraphNode{
            descriptor
        }
    }, |_,edge| {
        let modifier_event = edge.elems.iter()
            .find_map(|(key,val)| (*key == "label").then_some(val.trim_matches('\"')))
            .map(String::from);

        SpellGraphTransition {
            modifier_event,
        }
    });

    last_err?;
    let start_node = start_node.ok_or_else(|| String::from("No 'Start' node found in the spell"))?;
    
    Ok(Spell::new(start_node, resolved_graph))
}

pub fn parse_raw_graph(dotgraph: &str) -> Result<Graph<DotNodeWeight, DotAttrList>, String> {
    <Graph<DotNodeWeight, DotAttrList> as ParseFromDot>::try_from(dotgraph)
        .map_err(|e| e.to_string())
}
