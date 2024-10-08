import * as fs from 'fs';
import { parse, ObjectTypeDefinitionNode, InputObjectTypeDefinitionNode } from 'graphql';

/* UNDER CONSTRUCTION */

/* 
  FUNCIONES AUXILIARES
*/


export function getMutationFieldsFromGraphQLFile(filePath: string): string[] {
    const content = fs.readFileSync(filePath, 'utf8');
    const document = parse(content);

    let mutations: string[] = [];

    for (const definition of document.definitions) {
        if (definition.kind === 'ObjectTypeDefinition') {
            const objectTypeDefinition = definition as ObjectTypeDefinitionNode;
            if (objectTypeDefinition.name.value === 'Mutation') {
                for (const field of objectTypeDefinition.fields || []) {
                    mutations.push(field.name.value);
                }
            }
        }
    }
    return mutations;
}

export function getQueryFieldsFromGraphQLFile(filePath: string): string[] {
    const content = fs.readFileSync(filePath, 'utf8');
    const document = parse(content);

    let queries: string[] = [];

    for (const definition of document.definitions) {
        if (definition.kind === 'ObjectTypeDefinition') {
            const objectTypeDefinition = definition as ObjectTypeDefinitionNode;
            if (objectTypeDefinition.name.value === 'Query') {
                for (const field of objectTypeDefinition.fields || []) {
                    queries.push(field.name.value);
                }
            }
        }
    }
    return queries;
}

export function getInputFieldsFromGraphQLFile(filePath: string): string[] {
    const content = fs.readFileSync(filePath, 'utf8');
    const document = parse(content);

    let inputs: string[] = [];

    for (const definition of document.definitions) {
        if (definition.kind === 'InputObjectTypeDefinition') {
            const inputObjectTypeDefinition = definition as InputObjectTypeDefinitionNode;
            inputs.push(inputObjectTypeDefinition.name.value);
        }
    }

    return inputs;
}