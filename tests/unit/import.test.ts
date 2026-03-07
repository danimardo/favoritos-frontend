import { describe, it, expect } from 'vitest';

function validateImportJson(jsonStr: string): { valid: boolean; error?: string } {
    try {
        const data = JSON.parse(jsonStr);
        if (!Array.isArray(data)) {
            return { valid: false, error: 'El archivo debe ser un array de favoritos.' };
        }
        
        for (const item of data) {
            if (!item.url || !item.title) {
                return { valid: false, error: 'Cada favorito debe tener al menos un título y una URL.' };
            }
        }
        
        return { valid: true };
    } catch (e) {
        return { valid: false, error: 'El archivo no es un JSON válido.' };
    }
}

describe('Import Validation Utility', () => {
  it('T093: should validate correct JSON array of favorites', () => {
    const validJson = JSON.stringify([
      { title: 'Test 1', url: 'https://test1.com' },
      { title: 'Test 2', url: 'https://test2.com', tags: ['tag1'] }
    ]);
    expect(validateImportJson(validJson).valid).toBe(true);
  });

  it('should fail on invalid JSON syntax', () => {
    const invalidJson = '[{ title: "Test" }'; // missing closing ]
    expect(validateImportJson(invalidJson).valid).toBe(false);
    expect(validateImportJson(invalidJson).error).toBe('El archivo no es un JSON válido.');
  });

  it('should fail if not an array', () => {
    const notAnArray = JSON.stringify({ title: 'Test', url: 'https://test.com' });
    expect(validateImportJson(notAnArray).valid).toBe(false);
    expect(validateImportJson(notAnArray).error).toBe('El archivo debe ser un array de favoritos.');
  });

  it('should fail if required fields are missing', () => {
    const missingUrl = JSON.stringify([{ title: 'Test' }]);
    expect(validateImportJson(missingUrl).valid).toBe(false);
    expect(validateImportJson(missingUrl).error).toBe('Cada favorito debe tener al menos un título y una URL.');
  });
});
