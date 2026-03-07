import type { Favorito } from '../types/api';

const DB_NAME = 'favoritos_db';
const DB_VERSION = 1;
const STORE_NAME = 'favoritos';

export class DatabaseService {
    private db: IDBDatabase | null = null;

    async init(): Promise<void> {
        if (this.db) return;

        return new Promise((resolve, reject) => {
            const request = indexedDB.open(DB_NAME, DB_VERSION);

            request.onerror = () => reject('Error opening IndexedDB');
            request.onsuccess = () => {
                this.db = request.result;
                resolve();
            };

            request.onupgradeneeded = (event) => {
                const db = (event.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains(STORE_NAME)) {
                    db.createObjectStore(STORE_NAME, { keyPath: 'id' });
                }
            };
        });
    }

    async saveFavoritos(favoritos: Favorito[]): Promise<void> {
        await this.init();
        const db = this.db;
        if (!db) return;

        const transaction = db.transaction([STORE_NAME], 'readwrite');
        const store = transaction.objectStore(STORE_NAME);

        // Clear existing to sync
        store.clear();

        for (const favorito of favoritos) {
            store.add(favorito);
        }

        return new Promise((resolve, reject) => {
            transaction.oncomplete = () => resolve();
            transaction.onerror = () => reject('Error saving to IndexedDB');
        });
    }

    async getAllFavoritos(): Promise<Favorito[]> {
        await this.init();
        const db = this.db;
        if (!db) return [];

        return new Promise((resolve, reject) => {
            const transaction = db.transaction([STORE_NAME], 'readonly');
            const store = transaction.objectStore(STORE_NAME);
            const request = store.getAll();

            request.onsuccess = () => resolve(request.result);
            request.onerror = () => reject('Error reading from IndexedDB');
        });
    }

    async clear(): Promise<void> {
        await this.init();
        const db = this.db;
        if (!db) return;

        return new Promise((resolve, reject) => {
            const transaction = db.transaction([STORE_NAME], 'readwrite');
            const store = transaction.objectStore(STORE_NAME);
            const request = store.clear();

            request.onsuccess = () => resolve();
            request.onerror = () => reject('Error clearing IndexedDB');
        });
    }
}

export const databaseService = new DatabaseService();
