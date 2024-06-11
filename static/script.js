document.addEventListener('DOMContentLoaded', () => {
    const codeElement = document.getElementById('code');
    
    let codeText = `
#include <iostream>
using namespace std;

int main() {
    cout << "Hello, World!" << endl;
    return 0;
}
`.trim();
    
    let index = 0;

    function typeCode() {
        if (index < codeText.length) {
            codeElement.textContent += codeText.charAt(index);
            index++;
            setTimeout(typeCode, 50);
        } else {
            setTimeout(() => {
                codeElement.classList.add('fade-out');
                setTimeout(() => {
                    codeElement.style.display = 'none';
                    const helloWorldElement = document.getElementById('hello-world');
                    helloWorldElement.classList.remove('hidden');
                    const canvas = document.getElementById('particleCanvas');
                    canvas.classList.remove('hidden')
                    startParticleAnimation();
                }, 2000);
            }, 1000);
        }
    }

    typeCode();
});

function startParticleAnimation() {
    const canvas = document.getElementById('particleCanvas');
    const ctx = canvas.getContext('2d');

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    const particlesArray = [];
    const numberOfParticles = 100;

    class Particle {
        constructor(x, y, size, speedX, speedY) {
            this.x = x;
            this.y = y;

            this.size = size;
            this.speedX = speedX;
            this.speedY = speedY;
        }

        update() {
            this.x += this.speedX;
            this.y += this.speedY;

            if (this.x > canvas.width) this.x = 0;
            if (this.x < 0) this.x = canvas.width;
            if (this.y > canvas.height) this.y = 0;
            if (this.y < 0) this.y = canvas.height;
        }

        draw() {
            ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
            ctx.beginPath();
            ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
            ctx.closePath();
            ctx.fill();
        }
    }

    function init() {
        for (let i = 0; i < numberOfParticles; i++) {
            const size = Math.random() * 2 + 1;
            const x = Math.random() * canvas.width;
            const y = Math.random() * canvas.height;
            const speedX = (Math.random() * 2 - 1) * 0.5;
            const speedY = (Math.random() * 2 - 1) * 0.5;
            particlesArray.push(new Particle(x, y, size, speedX, speedY));
        }
    }

    function animate() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        particlesArray.forEach(particle => {
            particle.update();
            particle.draw();
        });
        requestAnimationFrame(animate);
    }

    init();
    animate();

    window.addEventListener('resize', () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        particlesArray.length = 0;
        init();
    });
}
