#version 430

layout(local_size_x = 32, local_size_y = 32) in;

layout(set = 0, binding = 0, rgba8) uniform writeonly image2D screen;

layout(push_constant) uniform PushConstantData {
    float iTime;
} pc;

layout(set = 1, binding = 0) buffer Data {
    uint voxels[];
};

const vec3 PPP = vec3(1, 1, 1);
const vec3 PNP = vec3(1, -1, 1);
const vec3 PNN = vec3(1, -1, -1);
const vec3 NPN = vec3(-1, 1, -1);
const vec3 NNN = vec3(-1, -1, -1);
const vec3 NNP = vec3(-1, -1, 1);
const vec3 NPP = vec3(-1, 1, 1);
const vec3 PPN = vec3(1, 1, -1);
const vec3 POS[8] = vec3[8](PNN, PNP, PPN, PPP, NNN, NNP, NPN, NPP);
struct Ray {
    vec3 o, d, invDir;
};
struct Hit {
    vec3 p;
    float t; // solution to p=o+t*d
    float tmax; //distance to exit point?
    float tmin; // distance to enter point?
    vec3 n; // normal
};
bool BBoxIntersect(const vec3 boxMin, const vec3 boxMax, const Ray r, out Hit hit) {
    vec3 tbot = r.invDir * (boxMin - r.o);
    vec3 ttop = r.invDir * (boxMax - r.o);
    vec3 tmin = min(ttop, tbot);
    vec3 tmax = max(ttop, tbot);
    vec2 t = max(tmin.xx, tmin.yz);
    float t0 = max(t.x, t.y);
    t = min(tmax.xx, tmax.yz);
    float t1 = min(t.x, t.y);
    hit.tmin = t0;
    hit.tmax = t1;
    return t1 > max(t0, 0.0);
}
vec4 trace(Ray ray, inout Hit hit) {
    vec3 center = vec3(0.0f);
    float scale = 1.0f;
    vec3 minBox = center - scale;
    vec3 maxBox = center + scale;
    vec4 f = vec4(0.0f);
    struct Stack {
        uint index;
        vec3 center;
        float scale;
    };
    Stack stack[10];
    int stackPos = 1;
    if (!BBoxIntersect(minBox, maxBox, ray, hit)) return f;
    uint index = 0u;
    scale *= 0.5f;
    stack[0] = Stack(0u, center, scale);
    while (stackPos-- > 0) {
        f = vec4(0.1f);
        center = stack[stackPos].center;
        index = stack[stackPos].index;
        scale = stack[stackPos].scale;
        uint voxel_node = voxels[index];
        uint voxel_group_offset = voxel_node >> 16;
        uint voxel_child_mask = (voxel_node & 0x0000FF00u) >> 8u;
        uint voxel_leaf_mask = voxel_node & 0x000000FFu;
        uint accumulated_offset = 0u;
        for (uint i = 0u; i < 8u; ++i) {
            bool empty = (voxel_child_mask & (1u << i)) == 0u;
            bool is_leaf = (voxel_leaf_mask & (1u << i)) != 0u;
            if (empty) { //empty
                continue;
            }

            vec3 new_center = center + scale * POS[i];
            vec3 minBox = new_center - scale;
            vec3 maxBox = new_center + scale;

            if (!BBoxIntersect(minBox, maxBox, ray, hit)) {
                if (!is_leaf) {
                    accumulated_offset += 1u;
                }
                continue;
            }
            if (is_leaf) { //not empty, but a leaf
                return vec4(1.0f, float(i) / 10.0, 0.0f, 1.0f);
            } else { //not empty and not a leaf
                stack[stackPos++] = Stack(voxel_group_offset + accumulated_offset, new_center, scale * 0.5f);
                f.z += 0.4f;
                accumulated_offset += 1u;
            }
        }
    }
    return f;
}
vec2 rotate2d(vec2 v, float a) {
    float sinA = sin(a);
    float cosA = cos(a);
    return vec2(v.x * cosA - v.y * sinA, v.y * cosA + v.x * sinA);
}

void main() {
    ivec2 fragCoord = ivec2(gl_GlobalInvocationID.xy);
    vec2 iResolution = vec2(imageSize(screen));

    vec2 screenPos = (fragCoord / iResolution) * 2.0 - 1.0;
    vec3 cameraDir = vec3(0.0, 0.0, 0.8);
    vec3 cameraPlaneU = vec3(1.0, 0.0, 0.0);
    vec3 cameraPlaneV = vec3(0.0, 1.0, 0.0) * iResolution.y / iResolution.x;
    vec3 rayDir = cameraDir + screenPos.x * cameraPlaneU + screenPos.y * cameraPlaneV;
    rayDir.y *= -1;
    vec3 rayPos = vec3(0.0, 0.25 * sin(pc.iTime * 2.7), -3.4);
    rayPos.xz = rotate2d(rayPos.xz, pc.iTime);
    rayDir.xz = rotate2d(rayDir.xz, pc.iTime);
    Ray ray;
    Hit hit;
    ray.o = rayPos;
    ray.d = rayDir;
    ray.invDir = 1.0f / rayDir;
    vec4 color = trace(ray, hit);

    voxels[0] = 129526u;

    if (length(color) > 0.5f) {
        imageStore(screen, fragCoord, color);
    } else {
        imageStore(screen, fragCoord, vec4(1.0));
    }
}
