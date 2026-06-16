(PaperWorkItem, PaperWorkResult, paperWorkItems, paperWorkFn, run_parallel) in "./PaperWork"

fn PaperWorkResult process(PaperWorkItem item)
    (id, payload) in item
    string output = ...
    PaperWorkResult result = (id, output)
    return result

fn void main()
    paperWorkItems items = [...]
    paperWorkResults results = run_parallel(items, process)