import re
import json
import statistics

def parse_sheet(file_path):
    with open(file_path, 'r') as f:
        content = f.read()

    # Regex to find rows. 
    # Structure: <tr ...><th ...>...</th><td ...>MRN</td><td ...>TIME</td><td ...>SYS</td>...</tr>
    
    # We'll just regex for the data in the <td> tags.
    # Because classes like 's0', 's1' change, we rely on position.
    # Columns: A=MRN, B=Time, C=Sys, D=Dia, E=HR, F=Pos, G=Pre/Post
    
    # Let's find all <tr> elements
    rows = re.findall(r'<tr.*?>(.*?)</tr>', content, re.DOTALL)
    
    data = []
    
    for row in rows:
        # Extract cell contents. Cells are <td>...</td>
        cells = re.findall(r'<td.*?>(.*?)</td>', row, re.DOTALL)
        if not cells: 
            continue
            
        # Clean cell content (remove div/span if any, though usually plain text or info)
        clean_cells = []
        for c in cells:
            # Remove HTML tags inside cell
            text = re.sub(r'<[^>]+>', '', c).strip()
            clean_cells.append(text)
            
        if len(clean_cells) < 7:
            continue
            
        # Check if header
        if clean_cells[0] == "PAT_MRN_ID":
            continue
            
        try:
            mrn = clean_cells[0]
            # time = clean_cells[1] 
            sys_bp = int(clean_cells[2]) if clean_cells[2].isdigit() else None
            dia_bp = int(clean_cells[3]) if clean_cells[3].isdigit() else None
            hr = int(clean_cells[4]) if clean_cells[4].isdigit() else None
            pos = clean_cells[5].lower()
            phase = clean_cells[6].lower() # pre or post
            
            if sys_bp and dia_bp and pos in ['lying', 'sitting', 'standing'] and phase in ['pre', 'post']:
                data.append({
                    "mrn": mrn,
                    "sys": sys_bp,
                    "dia": dia_bp,
                    "hr": hr,
                    "pos": pos,
                    "phase": phase
                })
        except ValueError:
            continue

    return data

def aggregate_data(data):
    # Group by MRN -> Phase -> Position -> Average
    # Result: { "mrn": { "pre": { "lying": {sys, dia, hr, count}, ... }, ... } }
    
    patients = {}
    
    for entry in data:
        mrn = entry['mrn']
        phase = entry['phase']
        pos = entry['pos']
        
        if mrn not in patients: patients[mrn] = {"pre": {}, "post": {}}
        if pos not in patients[mrn][phase]: 
            patients[mrn][phase][pos] = {"sys": [], "dia": [], "hr": []}
            
        patients[mrn][phase][pos]["sys"].append(entry['sys'])
        patients[mrn][phase][pos]["dia"].append(entry['dia'])
        if entry['hr']: patients[mrn][phase][pos]["hr"].append(entry['hr'])
        
    # Calculate averages
    results = []
    for mrn, phases in patients.items():
        p_data = {"mrn": mrn}
        for phase_name in ["pre", "post"]:
            p_data[phase_name] = {}
            for pos in ["lying", "sitting", "standing"]:
                stats = phases[phase_name].get(pos)
                if stats and stats["sys"]:
                   p_data[phase_name][pos] = {
                       "sys": round(statistics.mean(stats["sys"])),
                       "dia": round(statistics.mean(stats["dia"])),
                       "hr": round(statistics.mean(stats["hr"])) if stats["hr"] else None
                   }
                else:
                    p_data[phase_name][pos] = None
        results.append(p_data)
        
    return results

raw_data = parse_sheet('/Users/arcot/Documents/AG/valdisere 2026/Sheet1.html')
aggregated = aggregate_data(raw_data)
print(json.dumps(aggregated, indent=2))
