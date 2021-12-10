fn main() {
    // let input = _get_test_input();
    let input = _get_input();

    // ----------- Parse Input -----------

    let lines = input
        .trim()
        .split('\n')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    // ----------- Solve -----------

    let get_closing = |c: char| match c {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
        _ => ' ',
    };

    let get_value_p1 = |c: char| match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    };

    let get_value_p2 = |c: char| match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    };

    let mut p1 = 0;
    let mut p2_scores = vec!();
    for line in lines {
        let mut stack = vec![];
        let mut found_err = false;
        for c in line.chars() {
            if get_closing(c) != ' ' {
                stack.push(c);
            } else {
                let top = stack.pop();
                if top.is_some() && c != get_closing(top.unwrap()) {
                    p1 += get_value_p1(c);
                    found_err = true;
                    break;
                }
            }
        }
        if !found_err {
            let mut p2_score: i64 = 0;
            for c in stack.iter().rev().map(|c| get_closing(*c)) {
                p2_score = 5 * p2_score + get_value_p2(c);
            }
            p2_scores.push(p2_score);
        }
    }

    p2_scores.sort();
    let p2 = p2_scores[p2_scores.len() / 2];


    // ----------- Print -----------

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}

fn _get_test_input() -> String {
    return "

    [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]] 

"
    .to_string();
}

fn _get_input() -> String {
    return "

    [{[[{(((<[{{((()<>)(<><>))[{{}{}}((){})]}[<<()[]>[{}()]>[{<>{}}<()()>]]}{{[[{}<>]({}{})]<[{}
        <[({(([<[[<([[(){}]<()()>])>[[(({})[<>()])]]]<[(({()[]})[{<>{}>(<>{})])<({<>()}[<>{}])(({}<>){()<>
        ([{[([(((<<<{({}())<()[]>}<[[]()]{<>{}})>[{[[][]]{<><>}}]>>)[[[{{({}<>)([]<>)}}({(()[])<()[]>})](<[[()()]<{}
        (<([<<({<[[[([[]{})<()[]>)(<[]()>[{}<>])]<[{[]{}}<()[]>]<[<><>]>>]<{[({})[<>{}]]<[()[]](()<>)>}{((()
        {<({{<{<[<({([()<>]{<>[]})<<{}<>>>}((([]{})((){}))[[(){}](<>[])]))>][[<(<{()()>{{}[]}><{{}()}(()<>)>)
        {<([<<[<(<{<<(()[])[{}[]]>>(<{()[]}>({<>()}<[]()>))}>)>[{{{({<[]><(){}>]({{}[]}<{}<>>))<{<(){}><[]{}>}({<>{}}
        {[[([{<<{{(({{<>{}]<{}{}>}[<{}<>><()()>]))({({{}[]}<<>[]>)(<<>>{[][]})})}<[(({[]{}}[{}{}])<(<>())<(){
        [<(<[{[<{{[<[{{}}([][])]{({}{})(<>{}]}>{<(<>[])({}())>({[]<>}{[]()})}](<{[{}]<()[]>}<([][]){{}()}>>)}<<(<[()(
        <([[<{<{<[{[[{()}<[]>]]<(<[]()>)>}([([{}[]]<[]<>>)(<[][]><()<>))]{<<()[]>((){})>([{}<>]([]{}))})]{
        (<({[<((([<[{([]())<<><>>)](<{[]}[{}()]>[[{}<>]((){})])>{{{{{}<>}([][])}}}]{[(((<><>)<()()>)[([]<>
        <(([<(<{({{{<[()[]]<[]()>)[(<>)<<>>]}[[<(){}>[{}<>]]<[<>()]([]<>)>]}[<[{{}<>}([]{})]{[{}()]}>]})}>(({(
        <<(([[{{<[<[([()[]]<()[]>)<([][]){{}<>}>]<{[()[]]({}[])}<(<>[])>>>({<<[]()>{{}})}([<{}[]>[[]()]]))
        {{{<<[<[<<[<{(()[])[[]{}]}[[[]()]{{}<>}]>[{(<>)[<>()]}]]>(<{[[()<>]<{}[]>]({{}}[(){}])}{<<[]>(
        <([<<[{<[[{([([][])[{}()]][(<><>)[{}<>]])[(({}{}){{}<>})]}((({[][]}<()<>>)(({}{})[()<>])))][(<(({}<
        {[<{[([[{[{[<{()<>}<<>[]>>{{<>()}[<><>]}]}]([{([[][]](()()))<{[]<>}[()<>]>}{<<<>{}>{[]<>}>}))}([(<(((){}){(
        ({<{[{[[{({<<(()<>){<>()}>(<()>{{}()})>({[[]()][{}]}{{<>{}]{{}[]}})})}]<{{[{[<<>[]>][(<>())<<>()>]}<{<{
        ([(([[[{<({{{[()()]{<>{}}}}<[[[]<>][{}{}]]({()()}{()()})>}({[[(){}][[]{}]]{({}[])[{}()]}}{{[<><>]}
        ((<[<{(({(<<[{()()>]<(<>)<[]<>>>><{<{}[]>([]())}{{[]()}<{}<>>}>>)}))}[{(<({{{([][])[(){}]}}<<<{}<>>{{}[]}>(<{
        {<[[([<([<{([([]{}){{}<>}]([[]{}]({}())))[[<{}[]>{{}<>)]{({}[])[[]]}]}(([[[]{}][<>[]]](({}[]){{}{}}))
        [<<[<[({<{([<<<>{}>(()[])><({}[])[{}<>]>]<[<[][]>{()[]}}<[{}()]>>)<<<(()[])>>>}{<{<<{}()>[[]()]>{[<>(
        <(<(<[({{<<[[{()<>}[{}<>]][[[]()](()<>)]]{[<()>([]<>)]<[{}{}]<{}{}>>}>([[(()()){(){}}]]<[<[][]>{[]
        <[{[{{{{<({[[({}{})][<{}{}>{[]{}}]]{[[()()]{<>[]}]<[<><>]<<><>>>}})>{({(<<()()>(<>{})>{(()[
        <[(([(<<(<{<(<{}<>>{{}{}})[[[][]]([]())]>{{<()>[<>{}]}({<><>}({}()))}}>[([{(<><>)[()()]}[[{}[
        <((<[{([{[([({{}<>}<[][]>)[<[]<>>(<>{})]])(({[{}[]][<><>]}[{()[]}<{}{}>]))]<<{(<[][]>[<>]][{()(
        ((<[<<({{((<[(<>())[[]{}]]{{[][]}<<>>}>({<[]()>[[]{}]}[({}[])(()())])){({(<>[])[{}()]}[[{}{}>]){{[()<>]{
        {<<{{(({(<({({{}{}}<()<>>)}([[()[]]]))>)})[{([<<<<[]<>>>({()<>})>[[[{}<>]][(<><>)(<>[])]]>])}[(<<<{(
        {<[<{[<(([{[<<{}()>([]())>[{<>}{{}[]}]]}][{<[([]{})<()<>>]{{[]()>[{}[]]}>[<<<>()>[<>()]>{([]<>){[]()}}]}[
        <{{<{[<<[<<<(<{}[]><()[]>)<[()()]{()[]}>>{<<<>{}>{<>[]}>[<()()>{[]{}}]}><<{{[]{}}{{}{}}}>>>[{{{([]{})<{}[]>}}
        (<{(([[{[[[(<[{}()]<{}[]>>{<<>>({}{})})]]]{{{{(<[]>(())){<()[]>({}[])}}[[<(){}>[[]<>]]([{}{}]<[]{}>)]}}[<<{<[
        {{((({{[((((<<<>()><[]<>>>[[[]<>]<()()>])({(<>{})}[[{}<>]<{}[]>]))[<(([]())[{}<>])[<[]{}>({}<>)]>])[<[<
        {{((<([(([([([[]<>][{}<>])(([]()))])]<(((<[][]>{[]()})([{}{}]{<>()})))>))])[([{{[[[<[]{}>[<><>]]<[[]()]>]<(
        <(<[[((<[[{(<{[]{}}><[{}{}][{}[]]>)[<{[]()}[[]{})><[()<>]<{}()>>]}<[<{{}[]}{<>()}>{{<><>}{{}{}}
        {{[[<(<[<(<{[{[]}[{}<>]][((){})[()()]]}<([<>{}]{[]{}}){({}{})[<>]}>>{{{{<>[]}(())}{<{}{}><(){}>}}}
        (<<(([<[{<<[<(()[])>[{{}}([]<>)]][((()<>)<()<>>)<(<>){<>{}}>]>(<{[<>{}]<{}<>>}>)>}(([<<{<>[]}<<
        {[[{{([({<{{({{}()}<()[]>)[(<>{}){[]{}}]}}<(<{<><>}({}<>)><[{}()]<<>[]>>)>>{{<(<<>[]><{}[]>)>}[[<[[]
        [([{<<({<{<[<[[][]][{}{}]>({<><>})]<({<>{}})>><<<<<>[]>(()<>)>({()[]})>>}<([{<{}<>>{{}<>}}([<><>]{
        {[{({(([<[((<{[]()}{{}()}>{<()[]>[<>{}]})([[()[]]][{()()}([]{})])){(<{[]{}}[{}()]>[{<>[]}{{}()}]
        [{[{({[{{<(<<([]<>)[()[]]><<[]{}><<>{}>>>[([<>[]](()<>)}([{}{}](<>[]))])>[<(<[{}[]]><<<><>>[[]()]>)[{[()(
        [{<[<[[{{<{{((<>{})<[]{}>)(<<>[]>({}[]))}[<([]())<<><>>><<()<>>({}())>]}{<{[<>{}]}{([][]){[][]}}>({<[]{}>
        ({<((<([{{[[(([])[[]])[[<>{}][[]]]][[[[][]}{{}<>}]([{}{}][()<>])]]{[<[[][]]({}())>{[{}<>][<><>]}]}}}
        {<<{({[[({{{<(<>())(<>{})>(({}[])(<>{}))}<<{[]{}}<[]>>>}{{(<<>{}><{}{}>)(<()()><[]()>)}{[({}
        <[{{([<[{[((<[(){}][<>[]]>(<{}()>[<>[]]})[[(<>){<>()}]])<(([[][]]<<>{}>)){<[<><>][<>[]]><({}())
        ([(([((<<<{([{[]{}}])<[{()()}(()[])]{[()<>][<><>]}>>><{{[(()())<{}<>>]}(([[][]](<>[])))}((([
        [<(({{{(<({[[[()[]][()[]]]{[<><>]{[]<>}}>[([()()]([][]))[[{}{}]{[]{}}]]}{[[[<>][{}[]]]][{[<>()][()<
        [<[<<(([{{<<[((){})[<>{}]]{<[]()>}><{(()[])}((()[])[(){}])>>}}(({{[({}[])<()[]>]{[[][]]}}{(<<><>>)([(){}
        [{[[<(({{({[{<()<>>((){})}(({}{}))](<[()<>]({}{})><<<>{}>(<>())>)}{[<<<>()>{<><>}><(()[])<{}[]>>]))}<{[(
        <([[{{(([<<<<({}())([][])>[[[]{}]{[]<>}]>[{{()[]}<()[]>)[{{}[]}<[][]>]]>{<<[(){}](<>[])>[[{}[]]]>(
        [[[[({[{<{(<{[[]<>]<(){}>}([(){}>[{}{}])>[{[{}()]{(){}}}])}>}]<[<(<{[[[]()]<{}<>>](<[]><(){
        {(<[({{<<(<{{{{}{}}}<{<><>}<{}{}>>}<[<{}{}><()[]>]([{}[]]{<><>})>>)>>{[[([{<<><>>{()()}}<<<
        <[({<([{[{[({<{}{}><{}[]>})([<()()>{<><>}]([()[]]{{}[]})>]({<{()()}<[]()>>[{[]()}({}())]})
        <(([{(<([([([(()())[(){}]](<[]{}>[<><>]))(({()<>}<<>()>)(<{}[]>({}[])))][{[[[][]]](<<><>>)}
        <{{(<{<[{((((({})({}<>))(({}())(<>[]))){{{[][]}[(){}]}<({}<>)<{}<>>>})){<({[{}()]<[]<>>}((()[])(<>{
        {({({{{<((<{<{()[]}({}[])>{<{}<>>(()[])}}{([<>][[][]])}>)({({[<>()]<<>{}>}<[<>]([]{})>){([[]{
        {[<((({<([<({[<>](<>)}((<>[]))){[[()()][<>]](([]){{}{}})}>])(<[[[[{}()]][({}<>){<><>}]]{<[
        [<<<[(<<{({{<<()<>>({}{})><[<>[]]({}{})>}<{[(){}][(){}>}[[<>{}]{<>()}]>})[{[[[{}]<[]<>>][{(){}}]]<{[(
        [{((<[{<({[{<[[]<>][{}[]]>}[(([][])<<>()>){<{}{}>[{}()]}]][[{{()[]}<[]{}>}(({}<>)<{}[]>)]{<<[]<>><()()>}}]})>
        {[{{(<[{{{({{<{}{}><()>}[{[]()}{[]<>}]})}}[<{<[<{}()>[()[]]]<{{}<>}{[]}>>{(<{}{}><()()>)}}{{({[]()})([
        {<{{({([[<(<[<[]{}>{{}()}]<({}{}){(){}}>><([()()]<(){}>){([]())(()[])}>){([([]<>)]<{<><>}[()<>]>){{<[]<>>[(
        <[([(([[<[<[[<<>[]>[[]<>]]]<({[]<>}{<>{}})<[(){}]<[]<>>>>>[(<([]())>)[{[(){}](<>{})}<<[]<>>{<
        {<[(<<[{<(<<{({}{})[(){}]}{[{}[]]{[]()}}>{<<<>{}>{[]{}}>}>[<(<{}<>>){<(){}><{}<>>}>(<{<><>}{{}{}}><((){})
        ([(<[(({{({[<([][])[(){}]>{[[]()]}]<[{[][]}{{}[]}](<[]<>>{{}{}])>})}}({[([{(()())[[][]]}<((){})
        {[<[(<<{([({{<{}[]>(<>)}<(<>())<<><>>>}{(([]<>){<>()})})((({<>}<<>[]>)[([]()){{}()}]){<{<>()}>})](<<<[{}[
        <((<[{[[[<<(<{()[]}(()[])>((<><>)[{}{}])){(<<>()>{[][]})[<[]{}><()[]>]}>[{<<()[]>([][])><{[]{}}
        [{{({[<<{[<<{(()()){{}{}}}<[{}[]]{[]{}}>>>({({(){}}[<><>])({{}{}}<{}()>)}[{({}<>)<{}()>}])](([({()
        <(({([<[{(([(({}[])([]()))<<()<>>{[]<>})]<<<[][]>>(<{}<>><[]<>>)>){<[{<>}[<>[]]]{<{}<>>{[]<>}}>[<[<>[]][<>
        ([{<<[[[[([[<[[]]{<>[]}>{(()[])<(){}>}]<[<()[]>(<>())]({[]()}[[]()])>]<{<{()<>}<()[]>><<<>>{{}{}}>}>)
        ({{<<({(([<{(([][])[()[]])}[[[()<>](())])><[([{}()][{}<>])<[[]{}](()<>)>]>]{(<<((){})<(){}>>({
        [(<([(<{{<[[[{{}<>}{<>{}}]({()<>}<()[]>)]{[<<>[]>[{}<>]]<[{}[]](()[])>}]>}<{<<{{{}[]}((){})}>}}<{<[{()()}[<
        {<<[({[{{{[[{(()())({})}][<<<>{}>([]())>]][(([{}()]{()[]}))<({<>[]}[{}()])>]}[<(<<<><>>[[]{}]>)<<
        <{{{(([[[(([<(()())({}<>)>({[]{}}{(){}})]({{()}<[]()>}([[]{}]{()[]})))({{{()[]}<[]<>>}<[<>()]<<>[]}>}[{<[][]>
        [(<{<(([([<<<{[]()}<{}>>)<<{<>{}}{[]}>{<<>()>{{}<>}}>>][{<{[{}[]]}(<<>()>)>[{(<><>)({}<>)}<
        [[<<<[({[[<<((<><>){<><>})[(<><>)[<>()]]>{<[{}]>}>]]}<{<([<{<>{}}<<>()>>({[]<>}{()()})]({<{}{}>{(){}}}>)>[[(<
        {({[([{[[([<((()[])(()[]))<<{}()>{()()}>>({({}[])}[[<>()}<{}[]>])]<([{()<>}{<>()}]([()](<>{})))<<((
        <[((([<[<({{(<<>()>[<>[]])(([]()){[]<>))}(({()[]}<(){}>)[[[]()]<<><>>])}{[{{{}<>}[<>{}]}][(([]<>)((){}))]}
        ((<<[[[{((<<([[]<>][<>{}])[{<>{}}(()[])]>([{(){}}{{}<>}][({}{})([]())])>)<[{{{(){}}<[]{}>}(<[]<>>({}<>))
        [[[{({{[[<[<<<<><>>[[]<>]>[<[]<>><<>()>]>({{<>{}}})]{(<<<>{}>{{}{}}><(()[])[{}{}]>)}><<[{[()[]]{
        {[[{([[<<[[[{[()[]]{<>()}}({<><>})]{((<>[])[<>()])}]<(([[][]]<()[]>)){<({}<>)(<>{}}>{(<>{})(<><>)}}>]{([{[<><
        [([[<[<({[{(([()()]{{}[]})[([]())(()())])[{{[]()}[<>{}]}({{}[]}[<>()])]}]<([<({}[]){[]()}>(<()()>[()<>])][(
        <(<({{<(([[([[<>()]((){})]{(<>){<>[]}})[((<><>)({}{}))]]([<[{}[]]{[]<>}]][{<(){}><[]<>>}<[{}[
        {[[({<[[{[({<([][]){<>()}>}(<(()()){()<>}>{<<>{}>}))<{<<()[]><{}<>>><({}<>)({}())>}((({}{})
        (<(<({<<(<{[[<<>[]>({}{})][<[]()>{()()}]]}({{{<>[]}[<>{}]}({(){}}<()[]>)}}>)>{{{[({[[]()][[]{}]}(<<>[]>)){
        ({{{<(({[{{{{[{}<>][()<>]}<{()<>}>}[[({}())<[]()>][[<>[]]({}())]]}[[([{}()]({}{}))<([]())>]<{<{}[]>
        [{([({<{(<[({{()()}(()<>)}(({}<>)<[]{}>)){([[][]]<(){}>)}]<<<<<>()><{}<>>>[((){})[[]{}]]><{<[]{}>>>>>)[(
        {[(<({[<{<{{<{<>()}({}[])>(<{}[]>[<>()])}[[<[]<>>({})]{<<>{}><()[]>}]}>}[{{<{([]())[()()]}([[][]][
        {[{{{{<<<{<(([[][]]){{[]{}}({}{})})[([()()]{[]()}){([]{})<{}[]>}]>[<(<[][]>{[]<>})[{{}()}(<>(
        <[({{([[([{({{{}{}}<{}[]>}([(){}][<>{}]))}<{<[<>[]][[]<>]>[([][])]}([(<>{})[[][]]])>][{[{([]{})[[]<
        <{{{<(([({{({<{}<>>[()()]}(([]){{}}))}}[(({{[]()}<{}<>>}[{[][]}<[]{}>])[<[()()](()())><(()){<>{}}>])[{<{[]<>}
        {(<[{{[((<[[[[()()]([][])]([<>{}][<>()])]]>))<{(<<<([]{}){(){}}><<{}>{[][]}>>>{{(<<>>[(){}])<[<>()]{
        {[{({{(<{({<({<>}<[][]>)([[]{}]{[][]})>[{(()())(()<>)}]][<[<()[]><{}{}>]({<>[]}<{}[]>)>[{<{}{}>[{}{
        [{<{<({<<{[<((()<>)[{}()]){(<><>)<()<>>}>{(([]<>)[[]<>])<{()}[<><>]>}]}({[{{{}<>}({}())>(<{}<>>{{}()})][<
        ((<[{[[<[<(([(()[])<[]{}>](({}<>){<>[]}))<((()[])({}())){{()[]}[()<>]}>)[[((<>[])[<>()])[{{}<>}[{}{}]]]]><
        [{<({{(([([[<<()()><<><>>>[<()[]>([]<>)]]{[{()<>}]((()[]){{}{}})}])[({[([]<>)][[<>[]]<{}{}>]}{[[[]
        (<{(([{{<[([[<()<>>(<>)]<([])[(){}]>]){{[<[]<>>{{}{}}]}({[{}]}[{()[]}{{}<>}])}]>{<<{{<(){}>({}{}
        ({{(<{{[[<{<[{{}{}}{<>[]}][(<>{}>(()<>)]>}[{<{{}<>}><{()<>}<[]()>>}]>({[{{{}{}}<{}{}>}(((){})[[]<>])]({<[][]> 

"
    .to_string();
}