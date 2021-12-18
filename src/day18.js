const readInput = async (filename, fun = (x) => x) => {
  const fileStream = require("fs").createReadStream(filename);
  const rl = require("readline").createInterface({
    input: fileStream,
    crlfDelay: Infinity,
  });
  const items = [];
  for await (const line of rl) {
    items.push(fun(line));
  }
  return items;
};

const isNumber = x => typeof x === "number";
const isLeaf = x => !!(x.leaf);
const isNode = x => !(x.leaf);
const isLeft = x => x.parent.left == x;
const isRight = x => x.parent.right == x;

const parse = (n, parent = null) => {
  if (isNumber(n)) {
    return { leaf: true, n, parent };
  }
  const node = { parent };
  node.left = parse(n[0], node);
  node.right = parse(n[1], node);
  return node;
}

const sum = (x, y) => {
  const node = { left: x, right: y };
  x.parent = node;
  y.parent = node;
  return node;
}

function flat(x) {
  if (isLeaf(x)) {
    return [x];
  } else {
    return flat(x.left).concat(flat(x.right));
  }
}

const explode = (x, d = 0) => {
  if (
    isNode(x) && isLeaf(x.left) && isLeaf(x.right) && d === 4
  ) {
    // Values are all nodes, so also the 0 must be a node under the x's parent.
    const zero = parse(0, x.parent);
    if (isLeft(x)) {
      x.parent.left = zero;
    } else if (isRight(x)) {
      x.parent.right = zero;
    }
    return [zero, x.left.n, x.right.n];
  } else if (isNode(x)) {
    const left = explode(x.left, d + 1);
    if (left != null) return left;
    const right = explode(x.right, d + 1);
    if (right != null) return right;
  }
  return null;
}

const split = x => {
  if (isLeaf(x) && x.n >= 10) {
    const node = parse(
      [Math.floor(x.n / 2), Math.ceil(x.n / 2)],
      x.parent,
    );
    if (isLeft(x)) {
      x.parent.left = node;
    } else if (isRight(x)) {
      x.parent.right = node;
    } 
    return true;
  } else if (isNode(x)) {
    if (split(x.left)) return true;
    if (split(x.right)) return true;
  }
  return false;
}

const reduce = (tree) => {
  let done = false;
  while (!done) {
    done = true;
    const res = explode(tree);
    // Here we sum the values to the parent node. 
    // We flat the tree so it's easy 
    if (res != null) {
      const [zero, left, right] = res;
      const nodes = flat(tree);
      const pos = nodes.indexOf(zero);
      if (pos > 0) nodes[pos - 1].n += left;
      if (pos < nodes.length - 1) nodes[pos + 1].n += right;
      done = false;
    } else {
      done = !split(tree);
    }
  }
  return tree;
}

const magnitude = (tree) => {
  if (isLeaf(tree)) {
    return tree.n;
  }
  return 3 * magnitude(tree.left) + 2 * magnitude(tree.right);
}

const main = async () => {
  const lines = await readInput(process.argv[2], eval);
  let tree = parse(lines[0]);
  for (let i = 1; i < lines.length; i++) {
    tree = reduce(sum(tree, parse(lines[i])));
  }
  console.log("Solution part 1:", magnitude(tree));
  
  let res = 0;
  for (let i = 0; i < lines.length; i++) {
    for (let j = 0; j < lines.length; j++) {
      if (i == j) continue;
      const m = magnitude(
        reduce(sum(parse(lines[i]), parse(lines[j]))),
      );
      if (res < m) {
        res = m;
      }
    }
  }

  console.log("Solution part 2:", res);
}

main();
