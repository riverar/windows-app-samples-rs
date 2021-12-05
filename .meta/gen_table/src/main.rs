use reader::*;

fn main() {
    let reader = TypeReader::get_mut();
    include_all(&mut reader.types);

    let root = reader.types.get_namespace("Microsoft").unwrap();
    let mut trees = Vec::new();
    collect_trees(root.namespace, root, &mut trees);
    trees.sort_by_key(|f| f.namespace);

    println!("## API coverage\n");

    trees.iter().for_each(|tree| emit_markdown(tree));
}

fn emit_markdown(tree: &reader::TypeTree) {
    if !tree.types.is_empty() {
        println!("### {}", tree.namespace);

        println!("|   |   |\n|---|---|");
        tree.types
            .iter()
            .for_each(|(name, _)| println!("|{}| |", name));
        println!("|   |   |\n");
    }
}

fn include_all(tree: &mut reader::TypeTree) {
    tree.include = true;

    tree.types.values_mut().for_each(|entry| {
        entry.include = reader::TypeInclude::Full;
    });

    tree.namespaces.values_mut().for_each(include_all);
}

fn collect_trees<'a>(
    root: &'static str,
    tree: &'a reader::TypeTree,
    trees: &mut Vec<&'a reader::TypeTree>,
) {
    if tree.namespace == "Microsoft.Foundation" {
        return;
    };

    // Strip out flat C APIs from windows-app crate?
    if tree.namespace == "Microsoft.MRM" {
        return;
    };

    trees.push(tree);

    tree.namespaces
        .values()
        .for_each(|tree| collect_trees(root, tree, trees));
}
