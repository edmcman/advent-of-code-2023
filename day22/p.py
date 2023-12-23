#!/usr/bin/python

import functools
import sys
import tqdm

def brick_down(b, dz):
    a,b,i = b
    return ((a[0], a[1], a[2]-dz), (b[0], b[1], b[2]-dz), i)

def point_immediately_below(a, b):
    return a[0] == b[0] and a[1] == b[1] and a[2] +1 == b[2]

def point_below(a, b):
    return a[0] == b[0] and a[1] == b[1] and a[2] < b[2]

def point_to_ground(t):
    return t[2] - 1

def point_on_ground(t):
    return point_to_ground(t) == 0

def brick_to_ground(t):
    return min(point_to_ground(t[0]), point_to_ground(t[1]))

def brick_on_ground(t):
    return brick_to_ground(t) == 0

def points_in_brick(t):

    (a,b,_) = t

    if a[0] != b[0]:
        return ((x, a[1], a[2]) for x in range(a[0], b[0]+1))
    elif a[1] != b[1]:
        return ((a[0], y, a[2]) for y in range(a[1], b[1]+1))
    elif a[2] != b[2]:
        return ((a[0], a[1], z) for z in range(a[2], b[2]+1))
    else:
        return [(a[0], a[1], a[2])]

def brick_below(a, b):

    #print(a)
    #print(b)

    if a == b: return False

    #print(f"Brick below? {a} < {b}?")
    for p1 in points_in_brick(a):
        for p2 in points_in_brick(b):
            #print(f"Point below {p1},{p2}")
            if point_below(p1, p2):
                return True
    return False

def brick_supports(a, b):
    # a supports b if any of a's points are immediately below any of b's points

    if a == b: return False

    for p1 in points_in_brick(a):
        for p2 in points_in_brick(b):
            if point_immediately_below(p1, p2):
                return True
    return False

def parse():
    for i, line in enumerate(sys.stdin):
        line = line.strip()
        (a,b) = line.split("~")
        (a1,a2,a3) = [int(x) for x in a.split(",")]
        (b1,b2,b3) = [int(x) for x in b.split(",")]
        yield (a1, a2, a3), (b1, b2, b3), i

@functools.cache
def compute_location(bricks, brick):
    #print(f"compute_location: Processing brick {brick}")
    if brick_on_ground(brick):
        #print("brick is on ground")
        return brick
    else:
        bricks_below = [b for b in bricks if brick_below(b, brick)]
        #print(f"Bricks below {brick}: {bricks_below}")
        #other_bricks = [other_brick for other_brick in brick if other_brick != other_brick]

        # For each point in the x-y plane, we need to determine which block is
        # the highest but still below us.

        cool = []

        for p in points_in_brick(brick):
            for other_brick in bricks_below:
                for i, op in enumerate(points_in_brick(other_brick)):
                    if point_below(op, p):
                        cool.append((other_brick, op, p, p[2] - op[2], i))

        cool.sort(key=lambda x: x[3])

        closest = None

        if cool:
            closest = cool[0]

        distance_to_ground = brick_to_ground(brick)

        #print(f"{brick} Distance to ground: {distance_to_ground}")
        #print(f"{brick} Closest brick: {closest}")

        if closest is not None:
            # We are limited by closest.  Recurse
            #print("recursing!")
            closest_after_falling = compute_location(bricks, closest[0])

            point_after_falling = list(points_in_brick(closest_after_falling))[closest[4]]

            new_z = point_after_falling[2] + 1
            old_z = closest[2][2]
            dz = old_z - new_z
        else:
            dz = distance_to_ground

        #print(f"{brick} About to fall by {dz}")

        new_brick = brick_down(brick, dz)
        #print(f"{brick} Returning {new_brick}")

        return new_brick

def compute_locations(bricks):
    bricks_out = []
    for i, brick in enumerate(bricks):
        #print(f"Working on brick {brick}")
        new_brick = compute_location(tuple(bricks), brick)
        bricks_out.append(new_brick)

    return bricks_out

def p1(bricks):

    s = 0

    for brick in tqdm.tqdm(bricks):

        if False:

            brick_remove = [b for b in bricks if b != brick]
            #brick_copy = brick_remove.copy()
            print(f"About to test removing {brick}")

            if False:
                ok = True
                for b in brick_remove:
                    # Are there any bricks immediately below this brick?
                    any_supporters = any(ob for ob in brick_remove if brick_supports(ob, b))
                    on_ground = brick_on_ground(b)
                    if any_supporters or on_ground:
                        # not going anywhere
                        pass
                    else:
                        ok = False
                        break

                if ok:
                    s += 1
                    
            else:
                brick_new = compute_locations(brick_remove)
                if brick_new == brick_remove:
                    print(f"Removing {brick} did not change the board")
                    #print(brick_remove)
                    s += 1
                else:
                    print(f"Removing {brick} changed the board")
                    #print(brick_remove)
                    #print(brick_new)
                    pass

        elif True:

            # Find all the blocks that brick supports
            all_good = True
            supported_bricks = (b for b in bricks if brick_supports(brick, b))
            for supportee in supported_bricks:
                #print(f"Brick {brick} supports {supportee}")
                # Is there another supporter besides brick?
                other_supporters = list(b for b in bricks if b != brick and brick_supports(b, supportee))
                if other_supporters or brick_on_ground(supportee):                    
                    #print(f"Ah, but {supportee} has other supporters: {other_supporters}")
                    pass
                else:
                    #print(f"Brick {supportee} has no other supporters")
                    all_good = False
                    break

            if all_good:
                #print(f"Brick {brick} can be dissolved")
                s += 1
            else:
                #print(f"Brick {brick} can't be dissolved")
                pass


    return s

def main():

    sys.setrecursionlimit(1000000)

    bricks = list(parse())
    #print(bricks)

    new_bricks = compute_locations(bricks)

    with open("/tmp/bricks.txt", "w") as f:
        for brick in new_bricks:
            print(f"{brick[2]} {brick[0][0]},{brick[0][1]},{brick[0][2]} {brick[1][0]},{brick[1][1]},{brick[1][2]}", file=f)

    print(f"Here are the fallen bricks: {new_bricks}")

    print(f"p1 {p1(new_bricks)}")

main()