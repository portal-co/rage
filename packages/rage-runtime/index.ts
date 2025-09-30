const num = (mask: number, array: keyof typeof globalThis) =>
  class Num {
    #value: number;
    static #arrayType = globalThis[array];
    static #array(a: number[]): number[] {
      if (Num.#arrayType) {
        return new Num.#arrayType(a);
      }
      return a;
    }
    static array(a: number[]): number[] {
      return Num.#array(a);
    }
    constructor(a: number) {
      this.#value = a & mask;
    }
    get value() {
      return this.#value & mask;
    }
    set value(v) {
      this.#value = v & mask;
    }
  };
export let U8 = num(0xff, "Uint8Array");
export let U16 = num(0xffff, "Uint16Array");
export let U32 = num(0xffffffff, "Uint32Array");
export let I8 = num(0xff, "Uint8Array");
export let I16 = num(0xffff, "Uint16Array");
export let I32 = num(0xffffffff, "Uint32Array");
export class RTTI {
  #klass: { prototype: any };
  #args: RTTI[];
  static #cache: WeakMap<{ prototype: any }, RTTI> = new WeakMap();
  static #_WeakMap = WeakMap;
  static #_WeakMap_has = WeakMap.prototype.has.call.bind(WeakMap.prototype.has);
  static #_WeakMap_get = WeakMap.prototype.get.call.bind(WeakMap.prototype.get);
  static #_WeakMap_set = WeakMap.prototype.set.call.bind(WeakMap.prototype.set);

  static #_Map = Map;
  static #_Map_has = Map.prototype.has.call.bind(Map.prototype.has);
  static #_Map_get = Map.prototype.get.call.bind(Map.prototype.get);
  static #_Map_set = Map.prototype.set.call.bind(Map.prototype.set);
  static #_Map_delete = Map.prototype.delete.call.bind(Map.prototype.delete);

  #argCache: WeakMap<RTTI, RTTI> = new RTTI.#_WeakMap();
  constructor(klass: { prototype: any }, ...args: RTTI[]) {
    if (args.length) {
      let [firstArg, ...rest] = args;
      let cached = new RTTI(klass, ...rest);
      // if (RTTI.#argCache.has(cached)) {
      const g = cached.#argCache;
      if (RTTI.#_WeakMap_has(g, firstArg)) {
        return RTTI.#_WeakMap_get(g, firstArg)! as RTTI;
      }
      RTTI.#_WeakMap_set(g, firstArg, this);
      this.#klass = klass;
      this.#args = args;
    } else {
      if (RTTI.#_WeakMap_has(RTTI.#cache, klass))
        return RTTI.#_WeakMap_get(RTTI.#cache, klass)! as RTTI;
      this.#klass = klass;
      this.#args = args;
      RTTI.#_WeakMap_set(RTTI.#cache, klass, this);
    }
  }
  get klass() {
    return this.#klass;
  }
  get args() {
    return this.#args;
  }
  #traitImpls: Map<(a: RTTI) => boolean, any> = new RTTI.#_Map();
  #directImpls: WeakMap<{ prototype: any }, any> = new RTTI.#_WeakMap();
  *#structImpls(a: RTTI): Generator<any, void, void> {
    if (
      a.#args.length === 0 &&
      RTTI.#_WeakMap_has(this.#directImpls, a.#klass)
    ) {
      yield RTTI.#_WeakMap_get(this.#directImpls, a.#klass);
    } else {
      for (var [f, v] of this.#traitImpls) if (f(a)) yield v;
    }
  }
  *structImpls(a: RTTI): Generator<any, void, void> {
    yield* this.#structImpls(a);
  }
  *#impls(a: RTTI): Generator<any, void, void> {
    yield* a.#structImpls(this);
  }
  *impls(a: RTTI): Generator<any, void, void> {
    yield* this.#impls(a);
  }

  #is: (a: RTTI) => boolean = (a) => a === this;

  #implementOn(
    a: RTTI | ((a: RTTI) => boolean),
    v: any,
    { prune }: { prune?(a: () => void): void } = {}
  ): () => void {
    let del = () => {};
    if (a instanceof RTTI) {
      if (a.#args.length === 0) {
        RTTI.#_WeakMap_set(this.#directImpls, a.#klass, v);
      } else {
        RTTI.#_Map_set(this.#traitImpls, a.#is, v);
        del = () => RTTI.#_Map_delete(this.#traitImpls, a.#is);
      }
    } else {
      RTTI.#_Map_set(this.#traitImpls, a, v);
      del = () => RTTI.#_Map_delete(this.#traitImpls, a);
    }
    this.#hasImplCache = new RTTI.#_WeakMap();
    if (prune) prune(del);
    return del;
  }
  implementOn(
    a: RTTI | ((a: RTTI) => boolean),
    v: any,
    { prune }: { prune?(a: () => void): void } = {}
  ): () => void {
    return this.#implementOn(a, v, { prune });
  }

  #hasImplCache: WeakMap<RTTI, boolean> = new RTTI.#_WeakMap();
  #hasImpl(a: RTTI): boolean {
    if (RTTI.#_WeakMap_has(this.#hasImplCache, a))
      return RTTI.#_WeakMap_get(this.#hasImplCache, a) as any;
    if (
      a.#args.length === 0 &&
      RTTI.#_WeakMap_has(this.#directImpls, a.#klass)
    ) {
      RTTI.#_WeakMap_set(this.#hasImplCache, a, true);
      return true;
    }
    for (var [f, v] of this.#traitImpls)
      if (f(a)) {
        RTTI.#_WeakMap_set(this.#hasImplCache, a, true);
        return true;
      }
    RTTI.#_WeakMap_set(this.#hasImplCache, a, false);
    return false;
  }
  hasImpl(a: RTTI): boolean {
    return this.#hasImpl(a);
  }
}
