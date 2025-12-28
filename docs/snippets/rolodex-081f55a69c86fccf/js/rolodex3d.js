// Three.js Rolodex 3D Visualization
import * as THREE from 'three';

let scene, camera, renderer, rolodexGroup;
let cards = [];
let cardMeshes = [];
let currentAngle = 0;
let targetAngle = 0;
let animating = false;
let containerId = null;
let onCardClickCallback = null;
let raycaster, mouse;

const CARD_WIDTH = 5;
const CARD_HEIGHT = 3;
const CARD_DEPTH = 0.05;
const ROLODEX_RADIUS = 6;
let CARD_SPACING = 15; // degrees between cards, adjusted dynamically based on card count

export function initRolodex(containerIdParam) {
    containerId = containerIdParam;
    const container = document.getElementById(containerId);
    if (!container) {
        console.error('Rolodex container not found:', containerId);
        return;
    }

    // Scene setup
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x1a1a2e);

    // Camera setup - closer for better card visibility
    const aspect = container.clientWidth / container.clientHeight;
    camera = new THREE.PerspectiveCamera(50, aspect, 0.1, 1000);
    camera.position.set(0, 1, 8);
    camera.lookAt(0, 0, 0);

    // Renderer setup
    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(container.clientWidth, container.clientHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    // Raycaster for click detection
    raycaster = new THREE.Raycaster();
    mouse = new THREE.Vector2();

    // Lighting
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(5, 10, 7);
    scene.add(directionalLight);

    const pointLight = new THREE.PointLight(0x4fc3f7, 0.5);
    pointLight.position.set(-5, 5, 5);
    scene.add(pointLight);

    // Create rolodex group
    rolodexGroup = new THREE.Group();
    scene.add(rolodexGroup);

    // Create rolodex base/stand
    createRolodexStand();

    // Handle window resize
    window.addEventListener('resize', onWindowResize);

    // Handle wheel/touchpad gestures
    container.addEventListener('wheel', onWheel, { passive: false });

    // Handle click for card selection
    renderer.domElement.addEventListener('click', onCanvasClick);
    renderer.domElement.style.cursor = 'pointer';

    // Start animation loop
    animate();
}

function createRolodexStand() {
    // Base plate
    const baseGeometry = new THREE.BoxGeometry(6, 0.3, 4);
    const baseMaterial = new THREE.MeshPhongMaterial({
        color: 0x2d2d44,
        specular: 0x111111,
        shininess: 30
    });
    const base = new THREE.Mesh(baseGeometry, baseMaterial);
    base.position.y = -3;
    scene.add(base);

    // Side supports
    const supportGeometry = new THREE.CylinderGeometry(0.15, 0.15, 5, 16);
    const supportMaterial = new THREE.MeshPhongMaterial({
        color: 0x3d3d5c,
        specular: 0x222222,
        shininess: 50
    });

    const leftSupport = new THREE.Mesh(supportGeometry, supportMaterial);
    leftSupport.position.set(-2.5, -0.5, 0);
    scene.add(leftSupport);

    const rightSupport = new THREE.Mesh(supportGeometry, supportMaterial);
    rightSupport.position.set(2.5, -0.5, 0);
    scene.add(rightSupport);

    // Axle
    const axleGeometry = new THREE.CylinderGeometry(0.1, 0.1, 5.5, 16);
    axleGeometry.rotateZ(Math.PI / 2);
    const axleMaterial = new THREE.MeshPhongMaterial({
        color: 0x666688,
        specular: 0x444444,
        shininess: 80
    });
    const axle = new THREE.Mesh(axleGeometry, axleMaterial);
    axle.position.y = 2;
    scene.add(axle);
}

// Conveyor belt parameters
const BELT_HEIGHT = 5;     // Height of the conveyor belt (y-axis extent)
const BELT_DEPTH = 2.5;    // Depth of the conveyor belt (z-axis extent)
const FRONT_CARD_COUNT = 12; // Number of cards visible in front section

// Calculate position on conveyor belt
// Returns { y, z, rotationX } for a given position along the track
function getConveyorPosition(trackPosition) {
    // trackPosition is 0-1 representing position along the conveyor belt
    // 0 = front bottom, 0.25 = front top, 0.5 = back top, 0.75 = back bottom

    const angle = trackPosition * 2 * Math.PI;

    // Elongated ellipse: taller in y, flatter in z
    const y = BELT_HEIGHT * Math.sin(angle);
    const z = BELT_DEPTH * Math.cos(angle);

    // Card rotation to face camera (tangent to ellipse)
    const rotationX = -angle;

    return { y, z, rotationX };
}

// Calculate track position for a card index using hybrid spacing
function calculateTrackPosition(index, totalCards) {
    if (totalCards <= FRONT_CARD_COUNT) {
        // Few cards: spread evenly across front section (0 to 0.5)
        return (index / totalCards) * 0.5;
    }

    // Many cards: front cards get more space, back cards are compressed
    if (index < FRONT_CARD_COUNT) {
        // Front cards: spread across 0 to 0.5 (front visible section - more space)
        return (index / FRONT_CARD_COUNT) * 0.5;
    } else {
        // Back cards: compressed in 0.5 to 1.0 section
        const backIndex = index - FRONT_CARD_COUNT;
        const backCount = totalCards - FRONT_CARD_COUNT;
        return 0.5 + (backIndex / backCount) * 0.5;
    }
}

function createCard(cardData, index, totalCards) {
    const group = new THREE.Group();
    group.userData = { cardIndex: index, cardId: cardData.id };

    // Card geometry
    const geometry = new THREE.BoxGeometry(CARD_WIDTH, CARD_HEIGHT, CARD_DEPTH);

    // Create canvas texture for card content - higher resolution for clarity
    const canvas = document.createElement('canvas');
    canvas.width = 640;
    canvas.height = 400;
    const ctx = canvas.getContext('2d');

    // Card background - clean white
    ctx.fillStyle = '#ffffff';
    ctx.fillRect(0, 0, 640, 400);

    // Card border - darker for contrast
    ctx.strokeStyle = '#333';
    ctx.lineWidth = 4;
    ctx.strokeRect(2, 2, 636, 396);

    // Header bar - darker blue for contrast
    ctx.fillStyle = '#1976d2';
    ctx.fillRect(0, 0, 640, 65);

    // Name - larger, bolder, white on blue
    ctx.fillStyle = '#ffffff';
    ctx.font = 'bold 36px Arial';
    ctx.fillText(cardData.name || 'New Card', 20, 45);

    let yPos = 100;

    // Company - prominent
    if (cardData.company) {
        ctx.fillStyle = '#1976d2';
        ctx.font = 'bold 22px Arial';
        ctx.fillText(cardData.company, 20, yPos);
        yPos += 45;
    }

    // Contact info - high contrast black
    ctx.fillStyle = '#000000';
    ctx.font = '20px Arial';

    if (cardData.email) {
        ctx.fillText(cardData.email, 20, yPos);
        yPos += 35;
    }

    if (cardData.phone) {
        ctx.fillText(cardData.phone, 20, yPos);
        yPos += 35;
    }

    // Notes - dark gray
    if (cardData.notes) {
        ctx.fillStyle = '#444';
        ctx.font = '16px Arial';
        const notes = cardData.notes.substring(0, 50);
        ctx.fillText(notes, 20, yPos);
    }

    const texture = new THREE.CanvasTexture(canvas);
    texture.needsUpdate = true;

    // Materials - front has texture, back is plain
    const frontMaterial = new THREE.MeshPhongMaterial({
        map: texture,
        specular: 0x111111,
        shininess: 10
    });
    const backMaterial = new THREE.MeshPhongMaterial({
        color: 0xeeeeee,
        specular: 0x111111,
        shininess: 10
    });
    const sideMaterial = new THREE.MeshPhongMaterial({
        color: 0xdddddd
    });

    const materials = [
        sideMaterial,   // right
        sideMaterial,   // left
        sideMaterial,   // top
        sideMaterial,   // bottom
        frontMaterial,  // front
        backMaterial    // back
    ];

    const card = new THREE.Mesh(geometry, materials);
    card.userData = { cardIndex: index, cardId: cardData.id };
    group.add(card);

    // Position card on the conveyor belt
    const trackPos = calculateTrackPosition(index, totalCards);
    const pos = getConveyorPosition(trackPos);
    group.position.x = 0;
    group.position.y = pos.y + 1; // Offset to center vertically
    group.position.z = pos.z;
    group.rotation.x = pos.rotationX;

    // Store track position for navigation
    group.userData.trackPosition = trackPos;

    return group;
}

export function updateCards(cardsJson) {
    try {
        const newCards = JSON.parse(cardsJson);

        // Clear existing cards from the group
        while (rolodexGroup.children.length > 0) {
            const child = rolodexGroup.children[0];
            rolodexGroup.remove(child);
            if (child.geometry) child.geometry.dispose();
            if (child.material) {
                if (Array.isArray(child.material)) {
                    child.material.forEach(m => m.dispose());
                } else {
                    child.material.dispose();
                }
            }
        }

        cards = newCards;
        cardMeshes = [];

        // Create new cards (in same order as list)
        cards.forEach((cardData, index) => {
            const cardMesh = createCard(cardData, index, cards.length);
            cardMeshes.push(cardMesh);
            rolodexGroup.add(cardMesh);
        });

        // Reset to show first card
        currentCardIndex = 0;
        trackOffset = 0;
        targetTrackOffset = 0;

    } catch (e) {
        console.error('Error updating cards:', e);
    }
}

// Track current card index for navigation
let currentCardIndex = 0;
let trackOffset = 0;
let targetTrackOffset = 0;

// Reposition all cards based on track offset
function updateCardPositions() {
    cardMeshes.forEach((group, index) => {
        let trackPos = calculateTrackPosition(index, cards.length) - trackOffset;
        // Wrap around
        while (trackPos < 0) trackPos += 1;
        while (trackPos >= 1) trackPos -= 1;

        const pos = getConveyorPosition(trackPos);
        group.position.y = pos.y + 1;
        group.position.z = pos.z;
        group.rotation.x = pos.rotationX;
    });
}

export function rotateToCard(index) {
    if (index < 0 || index >= cards.length) return;
    currentCardIndex = index;
    // Calculate target offset to bring this card to the front (trackPos = 0)
    targetTrackOffset = calculateTrackPosition(index, cards.length);
    animating = true;
}

export function rotateNext() {
    if (currentCardIndex < cards.length - 1) {
        currentCardIndex++;
        rotateToCard(currentCardIndex);
    }
}

export function rotatePrev() {
    if (currentCardIndex > 0) {
        currentCardIndex--;
        rotateToCard(currentCardIndex);
    }
}

export function setCardClickCallback(callback) {
    onCardClickCallback = callback;
}

export function disposeRolodex() {
    if (renderer) {
        renderer.domElement.removeEventListener('click', onCanvasClick);
        renderer.dispose();
        const container = document.getElementById(containerId);
        if (container) {
            container.removeEventListener('wheel', onWheel);
            if (renderer.domElement) {
                container.removeChild(renderer.domElement);
            }
        }
    }
    window.removeEventListener('resize', onWindowResize);
}

function onWindowResize() {
    const container = document.getElementById(containerId);
    if (!container) return;

    camera.aspect = container.clientWidth / container.clientHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(container.clientWidth, container.clientHeight);
}

let scrollAccumulator = 0;
const scrollThreshold = 50; // Pixels of scroll to trigger card change

function onWheel(event) {
    event.preventDefault();

    // Accumulate scroll delta
    scrollAccumulator += event.deltaY;

    // Navigate when threshold is reached (inverted for natural scroll feel)
    if (Math.abs(scrollAccumulator) >= scrollThreshold) {
        if (scrollAccumulator > 0) {
            rotatePrev(); // Scroll down = previous card (natural direction)
        } else {
            rotateNext(); // Scroll up = next card
        }
        scrollAccumulator = 0;
    }
}

function onCanvasClick(event) {
    if (!renderer || !camera || cards.length === 0) return;

    const rect = renderer.domElement.getBoundingClientRect();
    mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
    mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

    raycaster.setFromCamera(mouse, camera);

    // Check intersection with card meshes
    const allMeshes = [];
    cardMeshes.forEach(group => {
        group.traverse(child => {
            if (child.isMesh) {
                allMeshes.push(child);
            }
        });
    });

    const intersects = raycaster.intersectObjects(allMeshes, false);

    if (intersects.length > 0) {
        const clickedMesh = intersects[0].object;
        const cardIndex = clickedMesh.userData.cardIndex;
        const cardId = clickedMesh.userData.cardId;

        if (cardIndex !== undefined && onCardClickCallback) {
            onCardClickCallback(cardIndex, cardId);
        }
    }
}

function animate() {
    requestAnimationFrame(animate);

    // Smooth conveyor belt animation
    if (animating) {
        const diff = targetTrackOffset - trackOffset;
        if (Math.abs(diff) < 0.0001) {
            trackOffset = targetTrackOffset;
            animating = false;
        } else {
            trackOffset += diff * 0.1;
        }
        updateCardPositions();
    }

    renderer.render(scene, camera);
}
