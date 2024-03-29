//scroll nav style
window.addEventListener('scroll', () => {
  document.querySelector('.header__container').classList.toggle('bar-opacity', window.scrollY > 0);
  document.querySelector('header').classList.toggle('hide-content', window.scrollY > 0)
})


//Gallery images populated with ajax request
var folder = "gallery";



//Modal Gallery previews
var img = document.querySelectorAll(".image-gallery li img");
var modal = document.getElementById("gallery-modal");
var modalImg = document.getElementById("gallery-modal-img");

img.forEach((imgA) => {
  imgA.onclick = function () {
    modal.style.display = "block";
    modalImg.src = this.src;
  }


  modal.onclick = function () {
    modal.style.display = "none"
  }
});



if (window.innerWidth <= 1024) {
  const nav = document.querySelector('.nav__hamburger');
  const header = document.querySelector('.header__container');
  const openNavBtn = document.querySelector('#nav__toggle-open');
  const closeNavBtn = document.querySelector('#nav__toggle-close');

  const openNav = () => {
    console.info("pressed the burgggg")
    nav.style.display = 'flex';
    openNavBtn.style.display = 'none';
    closeNavBtn.style.display = 'inline-block';
    header.classList.add('bar-opacity');
  }

  openNavBtn.addEventListener('click', openNav);

  const closeNav = () => {
    console.info("pressed the x")
    nav.style.display = 'none';
    openNavBtn.style.display = 'inline-block';
    closeNavBtn.style.display = 'none';

    if(window.scrollY === 0)
    header.classList.remove('bar-opacity');
  }

  closeNavBtn.addEventListener('click', closeNav);

  nav.querySelectorAll('div li a').forEach(navLink => {
    navLink.addEventListener('click', closeNav);
  });
}