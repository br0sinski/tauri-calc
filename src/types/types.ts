export interface CalculationResult {
    result: number;
    error?: string;
}

export type Operation = 'add' | 'sub' | 'mult' | 'div' | 'square'; 

export interface CalculationInput {
    a: number;
    b: number;
}