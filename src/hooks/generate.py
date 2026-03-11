hookdefs = []
copies = []
hookinits = []

def generate_hookdef(paramcount):
    name = f"hook{paramcount}"
    args = f"({", ".join([f"a{i + 1}" for i in range(paramcount)])})"
    return f"define_hook!({name}, {args});"

def generate_copy(paramcount, id):
    name = f"hook{paramcount}_{id}"
    orig = f"hook{paramcount}"
    args = f"({", ".join([f"a{i + 1}" for i in range(paramcount)])})"
    return f"define_hook_copy!({name}, {orig}, {args});"

def generate_hookinit(paramcount, id):
    name = f"hook{paramcount}_{id}"
    return f"            HookData {{ addr: {name} as *mut c_void, param_count: {paramcount} }}"

for paramcount in range(10):
    hookdef = generate_hookdef(paramcount)
    hookdefs.append(hookdef)

    inits = []
    for id in range(10):
        copy = generate_copy(paramcount, id)
        copies.append(copy)

        init = generate_hookinit(paramcount, id)
        inits.append(init)

    hookinit = f"pool.available.insert(0, vec![\n{",\n".join(inits)}\n        ]);"
    hookinits.append(hookinit)

with open("template.rs") as t:
    template = t.read()

    lines = template.replace("$$HOOKDEFS$$", "\n".join(hookdefs))
    lines = lines.replace("$$COPIES$$", "\n".join(copies))
    lines = lines.replace("$$HOOKINITS$$", "\n\n        ".join(hookinits))

    with open("generated.rs", "w") as f:
        f.write(lines)