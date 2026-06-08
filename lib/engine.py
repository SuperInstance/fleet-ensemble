"""Production engine for fleet MIDI service."""
import json

def process(ternary_vector, base_pitch=60, bpm=120):
    notes = [base_pitch]
    for v in ternary_vector:
        if v == 1: notes.append(notes[-1] + 4)
        elif v == -1: notes.append(notes[-1] - 4)
        else: notes.append(notes[-1])
    return {
        'notes': notes,
        'vector': ternary_vector,
        'length': len(notes),
        'density': sum(1 for x in ternary_vector if x != 0) / len(ternary_vector),
        'balance': (sum(1 for x in ternary_vector if x == 1) - 
                    sum(1 for x in ternary_vector if x == -1)) / len(ternary_vector)
    }

if __name__ == '__main__':
    result = process([1,0,-1,1,0,-1,1,1])
    print(json.dumps(result, indent=2))
