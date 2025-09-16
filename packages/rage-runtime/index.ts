const num = (mask: number) => class { #value: number; constructor(a: number) { this.#value = a & mask }; get value() { return this.#value }; set value(v) { this.#value = v & mask } }
export let U8 = num(0xff);
export let U16 = num(0xffff);
export let U32 = num(0xffffffff);
export let I8 = num(0xff);
export let I16 = num(0xffff);
export let I32 = num(0xffffffff);
export class RTTI {
    #klass: { prototype: any };
    #args: RTTI[];
    static #cache: WeakMap<{ prototype: any }, RTTI> = new WeakMap();
    #argCache: WeakMap<RTTI, RTTI> = new WeakMap();
    constructor(klass: { prototype: any }, ...args: RTTI[]) {
        if (args.length) {
            let [firstArg, ...rest] = args;
            let cached = new RTTI(klass, ...rest);
            // if (RTTI.#argCache.has(cached)) {
            const g = cached.#argCache;
            if (g.has(firstArg)) {
                return g.get(firstArg)!;
            }
            g.set(firstArg, this);
            this.#klass = klass;
            this.#args = args;

        } else {
            if (RTTI.#cache.has(klass)) return RTTI.#cache.get(klass)!;
            this.#klass = klass;
            this.#args = args;
            RTTI.#cache.set(klass, this);
        }
    }
    get klass() {
        return this.#klass;
    }
    get args() {
        return this.#args;
    }
}